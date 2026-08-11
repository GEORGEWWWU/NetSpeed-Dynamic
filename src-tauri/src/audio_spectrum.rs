use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use realfft::RealFftPlanner;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const FFT_LEN: usize = 1024;
const BAR_COUNT: usize = 7;

static SPECTRUM: Mutex<[f32; BAR_COUNT]> = Mutex::new([0.35; BAR_COUNT]);

#[tauri::command]
pub fn get_audio_spectrum() -> [f32; BAR_COUNT] {
    *SPECTRUM.lock().unwrap_or_else(|e| e.into_inner())
}

// ── 频谱分析器 ──────────────────────────────────────────
struct SpectrumAnalyzer {
    fft: Arc<dyn realfft::RealToComplex<f32>>,
    output: Vec<realfft::num_complex::Complex32>,
    input: Vec<f32>,
    input_len: usize,
    adaptive_max: [f32; BAR_COUNT],
}

impl SpectrumAnalyzer {
    fn new() -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(FFT_LEN);
        let output = fft.make_output_vec();
        Self {
            fft,
            output,
            input: vec![0.0; FFT_LEN],
            input_len: 0,
            adaptive_max: [0.1; BAR_COUNT],
        }
    }

    fn push_sample(&mut self, sample: f32) {
        self.input[self.input_len] = sample;
        self.input_len += 1;
        if self.input_len == FFT_LEN {
            self.process();
            self.input_len = 0;
        }
    }

    fn process(&mut self) {
        if self.fft.process(&mut self.input, &mut self.output).is_err() {
            return;
        }

        // 7 个对数频段 (bin 范围)，覆盖 ~90 Hz → ~24 kHz
        let ranges: [(usize, usize); BAR_COUNT] = [
            (2, 8),     // ~90 – 375 Hz   低频
            (8, 18),    // ~375 – 844 Hz  中低
            (18, 40),   // ~844 – 1875 Hz 中频
            (40, 90),   // ~1.9k – 4.2k   中高
            (90, 190),  // ~4.2k – 8.9k   高频
            (190, 350), // ~8.9k – 16.4k  亮频
            (350, 511), // ~16.4k – 24k   空气感
        ];

        let mut raw = [0.0f32; BAR_COUNT];
        for (j, &(start, end)) in ranges.iter().enumerate() {
            let sum: f32 = self.output[start..end].iter().map(|v| v.norm()).sum();
            let avg = sum / (end - start) as f32;

            // 自适应归一化：EMA 跟踪历史峰值，自动调整灵敏度
            self.adaptive_max[j] = self.adaptive_max[j] * 0.995 + avg.max(0.01) * 0.005;
            raw[j] = (avg / (self.adaptive_max[j] * 2.3)).clamp(0.0, 1.0);
        }

        // 对称"山丘"排列：两端低、中间高，视觉更饱满
        let final_bins = [
            raw[6] * 0.75, // 最右：空气感
            raw[4] * 0.85, // 右二：高频
            raw[1] * 0.95, // 右三：中低
            raw[2] * 1.0,  // 正中：中频（最高）
            raw[3] * 0.95, // 左三：中高
            raw[5] * 0.85, // 左二：亮频
            raw[0] * 0.75, // 最左：低频
        ];

        // 直接写入，不做后端平滑（CSS transition 已足够）
        if let Ok(mut s) = SPECTRUM.try_lock() {
            *s = final_bins;
        }
    }
}

// ── 启动监听 ──────────────────────────────────────────
pub fn start_monitor() {
    thread::spawn(|| {
        let host = cpal::default_host();

        // 外层循环：设备拔出 / 切换时自动重连
        loop {
            let device = match host.default_output_device() {
                Some(d) => d,
                None => {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            };
            let config = match device.default_output_config() {
                Ok(c) => c,
                Err(_) => {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            };

            let sample_format = config.sample_format();
            let stream_config: cpal::StreamConfig = config.config();
            let channels = stream_config.channels as usize;

            let mut analyzer = SpectrumAnalyzer::new();
            let err_fn = |err| eprintln!("Audio capture error: {}", err);

            let stream = match sample_format {
                cpal::SampleFormat::F32 => device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _: &_| {
                        // 多声道混为单声道
                        for chunk in data.chunks(channels) {
                            let mono = chunk.iter().sum::<f32>() / channels as f32;
                            analyzer.push_sample(mono);
                        }
                    },
                    err_fn,
                    None,
                ),
                cpal::SampleFormat::I16 => device.build_input_stream(
                    &stream_config,
                    move |data: &[i16], _: &_| {
                        for chunk in data.chunks(channels) {
                            let mono = chunk
                                .iter()
                                .map(|&s| s as f32 / i16::MAX as f32)
                                .sum::<f32>()
                                / channels as f32;
                            analyzer.push_sample(mono);
                        }
                    },
                    err_fn,
                    None,
                ),
                _ => {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            };

            match stream {
                Ok(s) => {
                    if s.play().is_err() {
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    }
                    // 保持线程存活
                    loop {
                        thread::sleep(Duration::from_secs(3600));
                    }
                }
                Err(_) => {
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    });
}
