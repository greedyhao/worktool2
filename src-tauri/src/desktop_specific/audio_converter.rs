// use serde::{Deserialize, Serialize};
// use std::path::Path;
// use tauri::{command, utils::config};
// use crate::wav_decoder::*;

// // OPUS 配置结构体
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct OpusConfig {
//     #[serde(rename = "sampleRate")]
//     pub sample_rate: String,
//     pub bitrate: String,
//     #[serde(rename = "frameSize")]
//     pub frame_size: String,
//     pub vbr: String,
// }

// // WAV 配置结构体
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct WavConfig {
//     #[serde(rename = "sampleRate")]
//     pub sample_rate: String,
//     #[serde(rename = "customSampleRate")]
//     pub custom_sample_rate: Option<u32>,
//     pub encoding: String,
// }

// // RAW 配置结构体
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct RawConfig {
//     #[serde(rename = "sampleRate")]
//     pub sample_rate: String,
//     #[serde(rename = "customSampleRate")]
//     pub custom_sample_rate: Option<u32>,
//     pub encoding: String,
// }

// // 音频格式枚举
// #[derive(Debug, Deserialize, Serialize, Clone)]
// #[serde(tag = "format")]
// pub enum AudioFormat {
//     #[serde(rename = "opus")]
//     Opus { config: OpusConfig },
//     #[serde(rename = "wav")]
//     Wav { config: WavConfig },
//     #[serde(rename = "raw")]
//     Raw { config: RawConfig },
// }

// // 音频转换选项
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct AudioConvertOptions {
//     pub format: String,
//     pub config: serde_json::Value, // 使用通用的 JSON Value 来接收不同的配置
// }

// // 解析后的音频配置
// #[derive(Debug, Clone)]
// pub struct ParsedAudioConfig {
//     pub format: AudioFormatType,
//     pub sample_rate: u32,
//     pub encoding: Option<AudioEncoding>,
//     pub opus_config: Option<ParsedOpusConfig>,
// }

// #[derive(Debug, Clone)]
// pub enum AudioFormatType {
//     Wav,
//     Opus,
//     Raw,
// }

// #[derive(Debug, Clone)]
// pub enum AudioEncoding {
//     S16,
//     S24,
//     S32,
//     U8,
// }

// #[derive(Debug, Clone)]
// pub struct ParsedOpusConfig {
//     pub bitrate: OpusBitrate,
//     pub frame_size_ms: f32,
//     pub vbr_enabled: bool,
// }

// #[derive(Debug, Clone)]
// pub enum OpusBitrate {
//     Fixed(u32),
//     Auto,
// }

// // 错误类型
// #[derive(Debug, thiserror::Error)]
// pub enum AudioConvertError {
//     #[error("不支持的音频格式: {0}")]
//     UnsupportedFormat(String),

//     #[error("无效的采样率: {0}")]
//     InvalidSampleRate(String),

//     #[error("无效的编码格式: {0}")]
//     InvalidEncoding(String),

//     #[error("无效的 OPUS 配置: {0}")]
//     InvalidOpusConfig(String),

//     #[error("文件不存在: {0}")]
//     FileNotFound(String),

//     #[error("参数解析错误: {0}")]
//     ParseError(String),

//     #[error("音频转换失败: {0}")]
//     ConversionFailed(String),
// }

// impl AudioEncoding {
//     pub fn from_str(encoding: &str) -> Result<Self, AudioConvertError> {
//         match encoding.to_lowercase().as_str() {
//             "s16" => Ok(AudioEncoding::S16),
//             "s24" => Ok(AudioEncoding::S24),
//             "s32" => Ok(AudioEncoding::S32),
//             "u8" => Ok(AudioEncoding::U8),
//             _ => Err(AudioConvertError::InvalidEncoding(encoding.to_string())),
//         }
//     }

//     pub fn bits_per_sample(&self) -> u32 {
//         match self {
//             AudioEncoding::S16 => 16,
//             AudioEncoding::S24 => 24,
//             AudioEncoding::S32 => 32,
//             AudioEncoding::U8 => 8,
//         }
//     }

//     pub fn is_signed(&self) -> bool {
//         match self {
//             AudioEncoding::S16 | AudioEncoding::S24 | AudioEncoding::S32 => true,
//             AudioEncoding::U8 => false,
//         }
//     }
// }

// impl OpusBitrate {
//     pub fn from_str(bitrate: &str) -> Result<Self, AudioConvertError> {
//         match bitrate.to_lowercase().as_str() {
//             "auto" => Ok(OpusBitrate::Auto),
//             rate => {
//                 let rate_num = rate.parse::<u32>().map_err(|_| {
//                     AudioConvertError::InvalidOpusConfig(format!("无效的码率: {}", rate))
//                 })?;

//                 // 验证码率范围
//                 if rate_num < 6000 || rate_num > 256000 {
//                     return Err(AudioConvertError::InvalidOpusConfig(format!(
//                         "码率超出范围 (6000-256000): {}",
//                         rate_num
//                     )));
//                 }

//                 Ok(OpusBitrate::Fixed(rate_num))
//             }
//         }
//     }
// }

// // 参数解析器
// pub struct AudioConfigParser;

// impl AudioConfigParser {
//     pub fn parse_config(
//         options: &AudioConvertOptions,
//     ) -> Result<ParsedAudioConfig, AudioConvertError> {
//         let format = Self::parse_format(&options.format)?;

//         match format {
//             AudioFormatType::Wav => Self::parse_wav_config(&options.config),
//             AudioFormatType::Opus => Self::parse_opus_config(&options.config),
//             AudioFormatType::Raw => Self::parse_raw_config(&options.config),
//         }
//     }

//     fn parse_format(format_str: &str) -> Result<AudioFormatType, AudioConvertError> {
//         match format_str.to_lowercase().as_str() {
//             "wav" => Ok(AudioFormatType::Wav),
//             "opus" => Ok(AudioFormatType::Opus),
//             "raw" => Ok(AudioFormatType::Raw),
//             _ => Err(AudioConvertError::UnsupportedFormat(format_str.to_string())),
//         }
//     }

//     fn parse_sample_rate(
//         sample_rate_str: &str,
//         custom_sample_rate: Option<u32>,
//     ) -> Result<u32, AudioConvertError> {
//         match sample_rate_str {
//             "other" => custom_sample_rate.ok_or_else(|| {
//                 AudioConvertError::InvalidSampleRate("自定义采样率未提供".to_string())
//             }),
//             rate_str => rate_str
//                 .parse::<u32>()
//                 .map_err(|_| AudioConvertError::InvalidSampleRate(rate_str.to_string())),
//         }
//     }

//     fn parse_wav_config(
//         config: &serde_json::Value,
//     ) -> Result<ParsedAudioConfig, AudioConvertError> {
//         let sample_rate_str = config["sampleRate"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少采样率".to_string()))?;

//         let custom_sample_rate = config["customSampleRate"].as_u64().map(|v| v as u32);
//         let sample_rate = Self::parse_sample_rate(sample_rate_str, custom_sample_rate)?;

//         let encoding_str = config["encoding"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少编码格式".to_string()))?;
//         let encoding = AudioEncoding::from_str(encoding_str)?;

//         Ok(ParsedAudioConfig {
//             format: AudioFormatType::Wav,
//             sample_rate,
//             encoding: Some(encoding),
//             opus_config: None,
//         })
//     }

//     fn parse_opus_config(
//         config: &serde_json::Value,
//     ) -> Result<ParsedAudioConfig, AudioConvertError> {
//         let sample_rate_str = config["sampleRate"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少采样率".to_string()))?;
//         let sample_rate = sample_rate_str
//             .parse::<u32>()
//             .map_err(|_| AudioConvertError::InvalidSampleRate(sample_rate_str.to_string()))?;

//         // 验证 OPUS 支持的采样率
//         match sample_rate {
//             8000 | 12000 | 16000 | 24000 | 48000 => {}
//             _ => {
//                 return Err(AudioConvertError::InvalidSampleRate(format!(
//                     "OPUS 不支持的采样率: {}",
//                     sample_rate
//                 )))
//             }
//         }

//         let bitrate_str = config["bitrate"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少码率".to_string()))?;
//         let bitrate = OpusBitrate::from_str(bitrate_str)?;

//         let frame_size_str = config["frameSize"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少帧长".to_string()))?;
//         let frame_size_ms = frame_size_str.parse::<f32>().map_err(|_| {
//             AudioConvertError::InvalidOpusConfig(format!("无效的帧长: {}", frame_size_str))
//         })?;

//         let vbr_str = config["vbr"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少 VBR 设置".to_string()))?;
//         let vbr_enabled = vbr_str == "on";

//         let opus_config = ParsedOpusConfig {
//             bitrate,
//             frame_size_ms,
//             vbr_enabled,
//         };

//         Ok(ParsedAudioConfig {
//             format: AudioFormatType::Opus,
//             sample_rate,
//             encoding: None,
//             opus_config: Some(opus_config),
//         })
//     }

//     fn parse_raw_config(
//         config: &serde_json::Value,
//     ) -> Result<ParsedAudioConfig, AudioConvertError> {
//         let sample_rate_str = config["sampleRate"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少采样率".to_string()))?;

//         let custom_sample_rate = config["customSampleRate"].as_u64().map(|v| v as u32);
//         let sample_rate = Self::parse_sample_rate(sample_rate_str, custom_sample_rate)?;

//         let encoding_str = config["encoding"]
//             .as_str()
//             .ok_or_else(|| AudioConvertError::ParseError("缺少编码格式".to_string()))?;
//         let encoding = AudioEncoding::from_str(encoding_str)?;

//         Ok(ParsedAudioConfig {
//             format: AudioFormatType::Raw,
//             sample_rate,
//             encoding: Some(encoding),
//             opus_config: None,
//         })
//     }
// }

// // Tauri 命令函数
// #[command]
// pub fn convert_audio(file_path: String, options: AudioConvertOptions) -> Result<String, String> {
//     // 验证文件存在
//     if !Path::new(&file_path).exists() {
//         return Err(format!("文件不存在: {}", file_path));
//     }

//     // 解析配置
//     let config = AudioConfigParser::parse_config(&options).map_err(|e| e.to_string())?;

//     println!("音频转换配置:");
//     println!("  输入文件: {}", file_path);
//     println!("  输出格式: {:?}", config.format);
//     println!("  采样率: {} Hz", config.sample_rate);

//     if let Some(encoding) = &config.encoding {
//         println!(
//             "  编码: {:?} ({} 位, {})",
//             encoding,
//             encoding.bits_per_sample(),
//             if encoding.is_signed() {
//                 "有符号"
//             } else {
//                 "无符号"
//             }
//         );
//     }

//     if let Some(opus_config) = &config.opus_config {
//         println!("  OPUS 配置:");
//         println!("    码率: {:?}", opus_config.bitrate);
//         println!("    帧长: {} ms", opus_config.frame_size_ms);
//         println!(
//             "    VBR: {}",
//             if opus_config.vbr_enabled {
//                 "开启"
//             } else {
//                 "关闭"
//             }
//         );
//     }

//     // 这里实现实际的音频转换逻辑
//     // 例如调用 FFmpeg 或其他音频处理库
//     let result = perform_audio_conversion(&file_path, &config).map_err(|e| e.to_string())?;

//     Ok(result)
// }

// // 实际的音频转换实现（示例）
// fn perform_audio_conversion(
//     input_path: &str,
//     config: &ParsedAudioConfig,
// ) -> Result<String, AudioConvertError> {
//     // 生成输出文件名
//     let input_path_obj = Path::new(input_path);
//     let stem = input_path_obj
//         .file_stem()
//         .and_then(|s| s.to_str())
//         .unwrap_or("output");

//     let extension = match config.format {
//         AudioFormatType::Wav => "wav",
//         AudioFormatType::Opus => "opus",
//         AudioFormatType::Raw => "raw",
//     };

//     let output_path = input_path_obj.with_file_name(format!("{}_converted.{}", stem, extension));
//     let output_path_str = output_path.to_string_lossy().to_string();

//     // 这里可以集成实际的音频转换库
//     // 例如：
//     // - 使用 FFmpeg 命令行工具
//     // - 使用 Rust 的音频处理库如 cpal, hound, opus 等
//     // - 调用系统的音频编解码器

//     match config.format {
//         AudioFormatType::Wav => {
//             convert_to_wav(input_path, &output_path_str, config);
//         }
//         AudioFormatType::Opus => {
//             convert_to_opus(input_path, &output_path_str, config);
//         }
//         AudioFormatType::Raw => {
//             convert_to_raw(input_path, &output_path_str, config);
//         }
//     }

//     Ok(output_path_str)
// }

// // 具体的转换函数（示例实现）
// fn convert_to_wav(
//     input_path: &str,
//     output_path: &str,
//     config: &ParsedAudioConfig,
// ) -> Result<(), AudioConvertError> {
//     println!("正在转换为 WAV 格式...");
//     println!("  输入: {}", input_path);
//     println!("  输出: {}", output_path);

//     // 这里实现 WAV 转换逻辑
//     // 可以使用 hound crate 或调用 FFmpeg

//     // 示例：模拟转换过程
//     // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

//     Ok(())
// }

// fn convert_to_opus(
//     input_path: &str,
//     output_path: &str,
//     config: &ParsedAudioConfig,
// ) -> Result<(), AudioConvertError> {
//     println!("正在转换为 OPUS 格式...");
//     println!("  输入: {}", input_path);
//     println!("  输出: {}", output_path);

//     if let Some(opus_config) = &config.opus_config {
//         println!("  使用 OPUS 配置: {:?}", opus_config);
//     }

//     use opusic_sys::*;
//     unsafe {
//         let mut packet = vec![];
//         let mut packet_size = 0;
//         let mut pcm = vec![];
//         let mut frame_size = 0;
//         let mut err = 0;
//         let encoder: *mut OpusEncoder = opus_encoder_create(
//             config.sample_rate as i32,
//             1,
//             OPUS_APPLICATION_RESTRICTED_LOWDELAY,
//             &mut err,
//         );
//         if err != OPUS_OK {
//             return Err(AudioConvertError::ParseError(err.to_string()));
//         }
//         if config.opus_config.is_none() {
//             return Err(AudioConvertError::ParseError("缺少 OPUS 配置".to_string()));
//         }
//         config.opus_config.as_ref().map(|opus_config| {
//             let bitrate = match opus_config.bitrate {
//                 OpusBitrate::Fixed(bitrate) => {
//                     opus_encoder_ctl(encoder, OPUS_SET_BITRATE_REQUEST, bitrate as i32);
//                     bitrate
//                 }
//                 OpusBitrate::Auto => {
//                     opus_encoder_ctl(encoder, OPUS_SET_BITRATE_REQUEST, OPUS_AUTO);
//                     0
//                 }
//             };
//             opus_encoder_ctl(
//                 encoder,
//                 OPUS_SET_VBR_REQUEST,
//                 opus_config.vbr_enabled as i32,
//             );
//             opus_encoder_ctl(encoder, OPUS_SET_COMPLEXITY_REQUEST, 0);

//             frame_size = config.sample_rate as usize * opus_config.frame_size_ms as usize / 1000;
//             // println!("frame_size: {}", frame_size);
//             pcm = vec![0; frame_size];

//             packet_size = frame_size * 2 / (config.sample_rate * 16 / bitrate) as usize;
//             // println!("packet_size: {}", packet_size);
//             packet = vec![0; packet_size];
//         });
//         let mut processor = OpusProcessor::new(encoder, packet_size);
//         let res = WavDecoder::process_by_chunks(input_path, frame_size, &mut processor);

//         println!("转换完成: {:?}", res);
//         opus_encoder_destroy(encoder);
//     }

//     Ok(())
// }

// fn convert_to_raw(
//     input_path: &str,
//     output_path: &str,
//     config: &ParsedAudioConfig,
// ) -> Result<(), AudioConvertError> {
//     println!("正在转换为 RAW 格式...");
//     println!("  输入: {}", input_path);
//     println!("  输出: {}", output_path);

//     // 这里实现 RAW 转换逻辑

//     // 示例：模拟转换过程
//     // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

//     Ok(())
// }

// use hound::WavSpec;
// use opusic_sys::*;
// // Opus 编码器实现
// struct OpusProcessor {
//     encoder: *mut OpusEncoder,
//     packet: Vec<u8>,
// }

// impl OpusProcessor {
//     pub fn new(encoder: *mut OpusEncoder, packet_size: usize) -> Self {
//         Self {
//             encoder,
//             packet: vec![0u8; packet_size],
//         }
//     }
// }

// impl AudioProcessor for OpusProcessor {
//     fn process(&mut self, samples: &[i16], spec: &WavSpec) -> Result<(), String> {
//         let frame_size = samples.len() / spec.channels as usize;
        
//         unsafe {
//             let result = opus_encode(
//                 self.encoder,
//                 samples.as_ptr(),
//                 frame_size as i32,
//                 self.packet.as_mut_ptr(),
//                 self.packet.len() as i32,
//             );
            
//             if result < 0 {
//                 return Err(format!("Opus encoding failed with error code: {}", result));
//             }
            
//             // 在这里处理编码后的数据 (self.packet[0..result as usize])
//             println!("Encoded {} bytes of Opus data", result);
//         }
        
//         Ok(())
//     }
// }
