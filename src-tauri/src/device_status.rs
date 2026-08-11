use serde::Serialize;
use std::collections::HashSet;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut;
use windows::core::BSTR;
use windows::Devices::Bluetooth::{BluetoothConnectionStatus, BluetoothDevice, BluetoothLEDevice};
use windows::Devices::Enumeration::DeviceInformation;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eMultimedia, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::NetworkManagement::WiFi::{
    dot11_phy_type_eht, dot11_phy_type_he, dot11_phy_type_ht, dot11_phy_type_vht,
    wlan_interface_state_connected, wlan_intf_opcode_current_connection, WlanCloseHandle,
    WlanEnumInterfaces, WlanFreeMemory, WlanOpenHandle, WlanQueryInterface,
    WLAN_CONNECTION_ATTRIBUTES, WLAN_INTERFACE_INFO, WLAN_INTERFACE_INFO_LIST,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
};

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStatus {
    network_name: Option<String>,
    network_type: Option<String>,
    network_signal: Option<u32>,
    audio_output_name: Option<String>,
    output_volume: Option<u32>,
    output_muted: bool,
    bluetooth_connected_count: Option<u32>,
}

#[tauri::command]
pub async fn get_device_status() -> DeviceStatus {
    tokio::task::spawn_blocking(collect_device_status)
        .await
        .unwrap_or_default()
}

fn collect_device_status() -> DeviceStatus {
    unsafe {
        // The command may run on different Tauri worker threads. Initializing COM on each
        // worker is harmless; an already initialized thread simply returns a status code.
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }

    let (network_name, network_type, network_signal) =
        read_wifi_connection().unwrap_or((None, None, None));
    let (audio_output_name, output_volume, output_muted) =
        read_default_audio_output().unwrap_or((None, None, false));

    DeviceStatus {
        network_name,
        network_type,
        network_signal,
        audio_output_name,
        output_volume,
        output_muted,
        bluetooth_connected_count: read_connected_bluetooth_count(),
    }
}

fn read_default_audio_output() -> Option<(Option<String>, Option<u32>, bool)> {
    unsafe {
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()?;
        let device = enumerator
            .GetDefaultAudioEndpoint(eRender, eMultimedia)
            .ok()?;
        let endpoint_volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None).ok()?;

        let volume = endpoint_volume
            .GetMasterVolumeLevelScalar()
            .ok()
            .map(|value| (value.clamp(0.0, 1.0) * 100.0).round() as u32);
        let muted = endpoint_volume
            .GetMute()
            .map(|value| value.as_bool())
            .unwrap_or(false);

        let name = device
            .OpenPropertyStore(STGM_READ)
            .ok()
            .and_then(|store| store.GetValue(&PKEY_Device_FriendlyName).ok())
            .and_then(|value| BSTR::try_from(&value).ok())
            .map(|value| value.to_string())
            .filter(|value| !value.trim().is_empty());

        Some((name, volume, muted))
    }
}

fn read_connected_bluetooth_count() -> Option<u32> {
    // Ask the Bluetooth APIs to build their official "Connected" selectors for both
    // Classic and LE devices. Resolve each result to its hardware address so one physical
    // device exposed by multiple services is still counted exactly once.
    let classic_selector = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .ok()?;
    let le_selector = BluetoothLEDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .ok()?;

    let mut addresses = HashSet::new();
    for (selector, is_le) in [(classic_selector, false), (le_selector, true)] {
        let devices = DeviceInformation::FindAllAsyncAqsFilter(&selector)
            .ok()?
            .get()
            .ok()?;
        for index in 0..devices.Size().ok()? {
            let Ok(device) = devices.GetAt(index) else {
                continue;
            };
            let Ok(id) = device.Id() else {
                continue;
            };
            let address = if is_le {
                BluetoothLEDevice::FromIdAsync(&id)
                    .ok()
                    .and_then(|operation| operation.get().ok())
                    .and_then(|device| device.BluetoothAddress().ok())
            } else {
                BluetoothDevice::FromIdAsync(&id)
                    .ok()
                    .and_then(|operation| operation.get().ok())
                    .and_then(|device| device.BluetoothAddress().ok())
            };
            if let Some(address) = address {
                addresses.insert(address);
            }
        }
    }
    Some(addresses.len() as u32)
}

fn read_wifi_connection() -> Option<(Option<String>, Option<String>, Option<u32>)> {
    unsafe {
        let mut negotiated_version = 0;
        let mut handle = HANDLE::default();
        if WlanOpenHandle(2, None, &mut negotiated_version, &mut handle) != 0 {
            return None;
        }

        let mut interface_list: *mut WLAN_INTERFACE_INFO_LIST = null_mut();
        if WlanEnumInterfaces(handle, None, &mut interface_list) != 0 || interface_list.is_null() {
            let _ = WlanCloseHandle(handle, None);
            return None;
        }

        let count = (*interface_list).dwNumberOfItems as usize;
        let interfaces = std::slice::from_raw_parts(
            (*interface_list).InterfaceInfo.as_ptr() as *const WLAN_INTERFACE_INFO,
            count,
        );
        let mut result = None;

        for interface in interfaces {
            if interface.isState != wlan_interface_state_connected {
                continue;
            }

            let mut data_size = 0;
            let mut data: *mut c_void = null_mut();
            let query_result = WlanQueryInterface(
                handle,
                &interface.InterfaceGuid,
                wlan_intf_opcode_current_connection,
                None,
                &mut data_size,
                &mut data,
                None,
            );
            if query_result != 0
                || data.is_null()
                || data_size < size_of::<WLAN_CONNECTION_ATTRIBUTES>() as u32
            {
                if !data.is_null() {
                    WlanFreeMemory(data as *const c_void);
                }
                continue;
            }

            let attributes = &*(data as *const WLAN_CONNECTION_ATTRIBUTES);
            let ssid = &attributes.wlanAssociationAttributes.dot11Ssid;
            let ssid_len = (ssid.uSSIDLength as usize).min(ssid.ucSSID.len());
            let mut network_name = String::from_utf8_lossy(&ssid.ucSSID[..ssid_len])
                .trim_matches(char::from(0))
                .trim()
                .to_string();
            if network_name.is_empty() {
                let profile_len = attributes
                    .strProfileName
                    .iter()
                    .position(|value| *value == 0)
                    .unwrap_or(attributes.strProfileName.len());
                network_name = String::from_utf16_lossy(&attributes.strProfileName[..profile_len]);
            }

            let phy = attributes.wlanAssociationAttributes.dot11PhyType;
            let network_type = if phy == dot11_phy_type_eht {
                "Wi-Fi 7"
            } else if phy == dot11_phy_type_he {
                "Wi-Fi 6"
            } else if phy == dot11_phy_type_vht {
                "Wi-Fi 5"
            } else if phy == dot11_phy_type_ht {
                "Wi-Fi 4"
            } else {
                "Wi-Fi"
            };
            let signal = attributes
                .wlanAssociationAttributes
                .wlanSignalQuality
                .min(100);

            result = Some((
                (!network_name.is_empty()).then_some(network_name),
                Some(network_type.to_string()),
                Some(signal),
            ));
            WlanFreeMemory(data as *const c_void);
            break;
        }

        WlanFreeMemory(interface_list as *const c_void);
        let _ = WlanCloseHandle(handle, None);
        result.or(Some((None, None, None)))
    }
}
