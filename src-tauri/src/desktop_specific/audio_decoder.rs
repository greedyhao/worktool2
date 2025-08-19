// use rsmpeg::{
//     avcodec::{AVCodec, AVCodecContext},
//     avformat::{AVFormatContextInput, AVInputFormat},
//     avutil::{AVFrame, AVSampleFormat},
//     error::RsmpegError,
//     ffi::{self, AVDictionary},
// };
// use std::io::Write;
// use std::{
//     ffi::CString,
//     os::raw::c_int,
//     path::Path,
//     ptr::{self, NonNull},
// };

// // 定义音频块处理器 trait
// trait AudioChunkProcessor {
//     /// 处理音频块数据
//     fn process_chunk(
//         &mut self,
//         data: &[u8],
//         sample_count: usize,
//         channels: u16,
//         sample_rate: u32,
//         sample_format: AVSampleFormat,
//     );
// }

// // 示例：简单的文件写入处理器
// struct FileWriter {
//     file: std::fs::File,
// }

// impl FileWriter {
//     fn new(path: &str) -> Result<Self, std::io::Error> {
//         Ok(Self {
//             file: std::fs::File::create(path)?,
//         })
//     }
// }

// impl AudioChunkProcessor for FileWriter {
//     fn process_chunk(
//         &mut self,
//         data: &[u8],
//         _sample_count: usize,
//         _channels: u16,
//         _sample_rate: u32,
//         _sample_format: AVSampleFormat,
//     ) {
//         let _ = self.file.write_all(data);
//     }
// }

// // 音频解码器配置
// pub struct AudioDecoderConfig<'a> {
//     pub input_format: Option<&'a AVInputFormat>,
//     pub options: Option<&'a mut AVDictionary>,
// }

// impl<'a> Default for AudioDecoderConfig<'a> {
//     fn default() -> Self {
//         Self {
//             input_format: None,
//             options: None,
//         }
//     }
// }

// // 音频解码器
// struct AudioDecoder<'a> {
//     format_context: AVFormatContextInput,
//     codec_context: AVCodecContext,
//     stream_index: usize,
//     processor: &'a mut dyn AudioChunkProcessor,
// }

// impl<'a> AudioDecoder<'a> {
//     /// 创建音频解码器
//     pub fn new(
//         path: &Path,
//         processor: &'a mut dyn AudioChunkProcessor,
//         config: AudioDecoderConfig,
//     ) -> Result<Self, Box<dyn std::error::Error>> {
//         // 将Path转换为CString
//         let c_path = CString::new(path.to_str().ok_or("Invalid path")?)?;

//         // 解包配置参数
//         let input_format = config.input_format;

//         // 打开输入文件
//         let mut format_context = AVFormatContextInput::open(&c_path, input_format, &mut None)
//             .map_err(|e| format!("Failed to open file: {:?}", e))?;

//         // 查找音频流
//         let (stream_index, codec) = format_context
//             .streams()
//             .iter()
//             .enumerate()
//             .find_map(|(i, stream)| {
//                 let codecpar = unsafe { *stream.codecpar };
//                 AVCodec::find_decoder(codecpar.codec_id).map(|codec| (i, codec))
//             })
//             .ok_or("No audio stream found")?;

//         // 创建解码器上下文
//         let stream = &format_context.streams()[stream_index];
//         let codecpar = unsafe { *stream.codecpar };
//         let mut codec_context = AVCodecContext::new(&codec);

//         unsafe {
//             // 复制编解码器参数到上下文
//             ffi::avcodec_parameters_to_context(codec_context.as_mut_ptr(), &codecpar);

//             // 设置请求的样本格式为交错格式
//             codec_context.as_mut_ptr().request_sample_fmt = ffi::AVSampleFormat::AV_SAMPLE_FMT_S16;
//         }

//         codec_context
//             .open(None)
//             .map_err(|e| format!("Failed to open codec: {:?}", e))?;

//         // 验证声道布局
//         let ch_layout = unsafe { &codec_context.as_ref().ch_layout };
//         if ch_layout.nb_channels == 0 {
//             return Err("Invalid channel layout".into());
//         }

//         Ok(Self {
//             format_context,
//             codec_context,
//             stream_index,
//             processor,
//         })
//     }

//     /// 解码音频并按指定块大小处理
//     pub fn decode_in_chunks(
//         &mut self,
//         chunk_size: usize,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let sample_rate = self.codec_context.sample_rate;
//         let ch_layout = unsafe { &self.codec_context.as_ref().ch_layout };
//         let channels = ch_layout.nb_channels as u16;
//         let sample_format = self.codec_context.sample_fmt;

//         // 计算每个样本的字节大小
//         let sample_size = sample_format.bytes_per_sample() as usize;
//         let frame_size = channels as usize * sample_size;

//         // 缓冲区用于存储跨帧的剩余数据
//         let mut buffer: Vec<u8> = Vec::with_capacity(chunk_size * frame_size);

//         // 循环读取数据包
//         while let Some(packet) = self
//             .format_context
//             .read_packet()
//             .map_err(|e| format!("Read packet error: {:?}", e))?
//         {
//             if packet.stream_index as usize != self.stream_index {
//                 continue;
//             }

//             // 发送数据包到解码器
//             self.codec_context
//                 .send_packet(Some(&packet))
//                 .map_err(|e| format!("Send packet error: {:?}", e))?;

//             // 接收解码后的帧
//             self.process_frames(&mut buffer, chunk_size, frame_size)?;
//         }

//         // 刷新解码器
//         self.codec_context
//             .send_packet(None)
//             .map_err(|e| format!("Send flush packet error: {:?}", e))?;

//         self.process_frames(&mut buffer, chunk_size, frame_size)?;

//         // 处理剩余的缓冲数据
//         if !buffer.is_empty() {
//             let sample_count = buffer.len() / frame_size;
//             self.processor.process_chunk(
//                 &buffer,
//                 sample_count,
//                 channels,
//                 sample_rate,
//                 sample_format,
//             );
//         }

//         Ok(())
//     }

//     /// 处理所有待解码帧
//     fn process_frames(
//         &mut self,
//         buffer: &mut Vec<u8>,
//         chunk_size: usize,
//         frame_size: usize,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let sample_rate = self.codec_context.sample_rate;
//         let ch_layout = unsafe { &self.codec_context.as_ref().ch_layout };
//         let channels = ch_layout.nb_channels as u16;
//         let sample_format = self.codec_context.sample_fmt;

//         loop {
//             match self.codec_context.receive_frame() {
//                 Ok(frame) => {
//                     // 获取音频数据
//                     let data_ptr = frame.data[0];
//                     let nb_samples = frame.nb_samples as usize;
//                     let data_size = nb_samples * frame_size;

//                     // 将数据添加到缓冲区
//                     unsafe {
//                         let data_slice = std::slice::from_raw_parts(data_ptr, data_size);
//                         buffer.extend_from_slice(data_slice);
//                     }

//                     // 处理完整的块
//                     while buffer.len() >= chunk_size * frame_size {
//                         let chunk_bytes = chunk_size * frame_size;
//                         let chunk_data = &buffer[..chunk_bytes];

//                         self.processor.process_chunk(
//                             chunk_data,
//                             chunk_size,
//                             channels,
//                             sample_rate,
//                             sample_format,
//                         );

//                         // 移除已处理的数据
//                         buffer.drain(..chunk_bytes);
//                     }
//                 }
//                 Err(RsmpegError::DecoderDrainError) | Err(RsmpegError::DecoderFlushedError) => {
//                     break;
//                 }
//                 Err(e) => return Err(format!("Receive frame error: {:?}", e).into()),
//             }
//         }

//         Ok(())
//     }
// }

// // // 使用示例
// // fn main() -> Result<(), Box<dyn std::error::Error>> {
// //     // 创建处理器
// //     let mut file_writer = FileWriter::new("output.pcm")?;

// //     // 使用默认配置
// //     let config = AudioDecoderConfig::default();

// //     // 创建解码器
// //     let mut decoder = AudioDecoder::new(
// //         Path::new("input.mp3"),
// //         &mut file_writer,
// //         config,
// //     )?;

// //     // 按1024个样本的块大小解码
// //     decoder.decode_in_chunks(1024)?;

// //     Ok(())
// // }

// // // 配置选项的示例
// // fn example_with_options() -> Result<(), Box<dyn std::error::Error>> {
// //     use rsmpeg::avformat::av_find_input_format;

// //     // 创建处理器
// //     let mut file_writer = FileWriter::new("output.pcm")?;

// //     // 创建配置
// //     let mut options: Option<AVDictionary> = None;
// //     let mut options_ptr = options.as_mut().map(|d| d as *mut AVDictionary);

// //     // 设置一些选项
// //     unsafe {
// //         ffi::av_dict_set(&mut options_ptr, CString::new("probesize")?.as_ptr(), CString::new("4096")?.as_ptr(), 0);
// //         ffi::av_dict_set(&mut options_ptr, CString::new("analyzeduration")?.as_ptr(), CString::new("1000000")?.as_ptr(), 0);
// //     }

// //     let config = AudioDecoderConfig {
// //         input_format: av_find_input_format(CString::new("mp3")?.as_ptr()),
// //         options: options_ptr.map(|p| unsafe { *Box::from_raw(p) }),
// //     };

// //     // 创建解码器
// //     let mut decoder = AudioDecoder::new(
// //         Path::new("input.mp3"),
// //         &mut file_writer,
// //         config,
// //     )?;

// //     // 解码处理
// //     decoder.decode_in_chunks(1024)?;

// //     Ok(())
// // }
