Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Runtime.WindowsRuntime
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class NsdKugouWindow {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hwnd, out RECT rect);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
}
'@

function Await-WinRt($operation, [Type]$resultType) {
  $method = [System.WindowsRuntimeSystemExtensions].GetMethods() |
    Where-Object { $_.Name -eq 'AsTask' -and $_.IsGenericMethodDefinition -and $_.GetParameters().Count -eq 1 } |
    Select-Object -First 1
  $task = $method.MakeGenericMethod($resultType).Invoke($null, @($operation))
  $task.Result
}

[NsdKugouWindow]::SetProcessDPIAware() | Out-Null
$player = Get-Process KuGou -ErrorAction SilentlyContinue |
  Where-Object { $_.MainWindowHandle -ne 0 } |
  Select-Object -First 1
if (-not $player) { exit 1 }

$rect = New-Object NsdKugouWindow+RECT
if (-not [NsdKugouWindow]::GetWindowRect($player.MainWindowHandle, [ref]$rect)) { exit 1 }
$width = $rect.Right - $rect.Left
$height = $rect.Bottom - $rect.Top
if ($width -le 0 -or $height -le 0) { exit 1 }

$image = New-Object Drawing.Bitmap $width, $height
$graphics = [Drawing.Graphics]::FromImage($image)
$graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, (New-Object Drawing.Size $width, $height))
$cropWidth = [int]($width * .38); $cropHeight = [int]($height * .145)
$crop = New-Object Drawing.Bitmap $cropWidth, $cropHeight
$cropGraphics = [Drawing.Graphics]::FromImage($crop)
$cropGraphics.DrawImage($image, (New-Object Drawing.Rectangle 0, 0, $cropWidth, $cropHeight), (New-Object Drawing.Rectangle ([int]($width * .10)), ([int]($height * .85)), $cropWidth, $cropHeight), [Drawing.GraphicsUnit]::Pixel)
$path = Join-Path $env:TEMP 'nsd-kugou-timeline.png'
$crop.Save($path, [Drawing.Imaging.ImageFormat]::Png)
$cropGraphics.Dispose(); $graphics.Dispose(); $crop.Dispose(); $image.Dispose()

$fileType = [Windows.Storage.StorageFile, Windows.Storage, ContentType = WindowsRuntime]
$streamType = [Windows.Storage.Streams.IRandomAccessStream, Windows.Storage.Streams, ContentType = WindowsRuntime]
$decoderType = [Windows.Graphics.Imaging.BitmapDecoder, Windows.Graphics.Imaging, ContentType = WindowsRuntime]
$bitmapType = [Windows.Graphics.Imaging.SoftwareBitmap, Windows.Graphics.Imaging, ContentType = WindowsRuntime]
$ocrType = [Windows.Media.Ocr.OcrEngine, Windows.Foundation, ContentType = WindowsRuntime]
$file = Await-WinRt ($fileType::GetFileFromPathAsync($path)) $fileType
$stream = Await-WinRt ($file.OpenAsync([Windows.Storage.FileAccessMode]::Read)) $streamType
$decoder = Await-WinRt ($decoderType::CreateAsync($stream)) $decoderType
$bitmap = Await-WinRt ($decoder.GetSoftwareBitmapAsync()) $bitmapType
$result = Await-WinRt (($ocrType::TryCreateFromUserProfileLanguages()).RecognizeAsync($bitmap)) ([Windows.Media.Ocr.OcrResult, Windows.Media.Ocr, ContentType = WindowsRuntime])
$stream.Dispose()

$match = [regex]::Match($result.Text, '(\d{1,2})\s*[:：]\s*(\d{2})\s*[/／]')
if (-not $match.Success) { exit 1 }
$seconds = ([int]$match.Groups[1].Value * 60) + [int]$match.Groups[2].Value
Write-Output ($seconds * 1000)
