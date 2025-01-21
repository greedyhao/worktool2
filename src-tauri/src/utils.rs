use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
pub enum FileEncoding {
    UTF8,
    UTF16LE,
    UTF16BE,
    UTF32LE,
    UTF32BE,
    Unknown,
}

pub fn detect_encoding(file_path: &str) -> io::Result<FileEncoding> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;

    match buffer {
        [0xEF, 0xBB, 0xBF, _] => Ok(FileEncoding::UTF8),
        [0xFF, 0xFE, 0x00, 0x00] => Ok(FileEncoding::UTF32LE),
        [0x00, 0x00, 0xFE, 0xFF] => Ok(FileEncoding::UTF32BE),
        [0xFF, 0xFE, _, _] => Ok(FileEncoding::UTF16LE),
        [0xFE, 0xFF, _, _] => Ok(FileEncoding::UTF16BE),
        _ => Ok(FileEncoding::Unknown),
    }
}

/// 逐行读取文件并过滤出 ASCII 字符，流式处理
///
/// # 参数
/// - `file_path`: 文件路径
/// - `process_line`: 处理每行的回调函数
///
/// # 返回值
/// - `Ok(())`: 处理成功
/// - `Err(io::Error)`: 文件读取错误
pub fn process_ascii_lines_from_file<F>(file_path: &str, mut process_line: F) -> io::Result<()>
where
    F: FnMut(String) -> bool,
{
    let mut file = File::open(file_path)?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let encoding = detect_encoding(file_path)?;
    let (decoded_str, _, _) = match encoding {
        FileEncoding::UTF16BE => encoding_rs::Encoding::for_label(b"utf-16be")
            .unwrap()
            .decode(&buf),
        FileEncoding::UTF16LE => encoding_rs::Encoding::for_label(b"utf-16le")
            .unwrap()
            .decode(&buf),
        FileEncoding::UTF32BE => encoding_rs::Encoding::for_label(b"utf-32be")
            .unwrap()
            .decode(&buf),
        FileEncoding::UTF32LE => encoding_rs::Encoding::for_label(b"utf-32le")
            .unwrap()
            .decode(&buf),
        _ => encoding_rs::Encoding::for_label(b"utf-8")
            .unwrap()
            .decode(&buf),
    };

    for line in decoded_str.lines() {
        let ascii_line: String = line
            .chars()
            .filter(|&c| c.is_ascii()) // 只保留 ASCII 字符
            .collect();

        // 如果闭包返回 false，提前退出
        if !process_line(ascii_line) {
            break;
        }
    }

    Ok(())
}
