use hound::{WavReader, SampleFormat, WavSpec};
use std::path::Path;


// 定义音频处理器 trait
pub trait AudioProcessor {
    fn process(&mut self, samples: &[i16], spec: &WavSpec) -> Result<(), String>;
}

// WAV 解码器
pub struct WavDecoder;

impl WavDecoder {
    pub fn process_by_chunks<P: AudioProcessor>(
        wav_path: &str,
        chunk_size: usize,
        processor: &mut P,
    ) -> Result<(), hound::Error> {
        let mut reader = WavReader::open(wav_path)?;
        let spec = reader.spec();
        
        Self::validate_wav_format(&spec)?;
        
        let samples_per_chunk = chunk_size * spec.channels as usize;
        let mut buffer = vec![0i16; samples_per_chunk];
        let mut buffer_pos = 0;
        
        for sample_result in reader.samples::<i16>() {
            let sample = sample_result?;
            
            buffer[buffer_pos] = sample;
            buffer_pos += 1;
            
            if buffer_pos == samples_per_chunk {
                processor.process(&buffer, &spec)
                    .map_err(|e| hound::Error::FormatError("Error processing audio chunk:"))?;
                buffer_pos = 0;
            }
        }
        
        // 处理剩余不足一个 chunk 的样本
        if buffer_pos > 0 {
            processor.process(&buffer[..buffer_pos], &spec)
                .map_err(|e| hound::Error::FormatError("Error processing audio chunk:"))?;
        }
        
        Ok(())
    }

    fn validate_wav_format(spec: &WavSpec) -> Result<(), hound::Error> {
        if spec.sample_format != SampleFormat::Int {
            return Err(hound::Error::FormatError("Only integer PCM supported"));
        }
        if spec.bits_per_sample != 16 {
            return Err(hound::Error::FormatError("Only 16-bit samples supported"));
        }
        if spec.channels != 1 {
            return Err(hound::Error::FormatError("Only mono audio supported"));
        }
        Ok(())
    }
}

