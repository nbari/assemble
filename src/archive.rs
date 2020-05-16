use blake2s_simd::blake2sp;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::path::PathBuf;
use std::string::String;
use xz2::write::XzEncoder;

#[derive(Debug)]
pub struct Pack {
    pub file: PathBuf,
    pub checksum: String,
}

// compress using XZ and get the checksum (of the uncompressed file)
pub fn pack(file_path: &str, tmp_dir: &Path) -> Result<Pack, Box<dyn Error>> {
    let file = Path::new(file_path);
    let tmp_file_path = tmp_dir.join(format!("{}.xz", file_path));
    let mut compress: bool = true;
    if file.extension().and_then(|s| s.to_str()) == Some("xz") {
        compress = false;
    }
    let file = fs::File::open(file)?;
    let output_file = fs::File::create(&tmp_file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake2sp::State::new();
    let mut encoder = XzEncoder::new(output_file, 9);
    loop {
        let consummed = {
            let buffer = reader.fill_buf()?;
            if buffer.is_empty() {
                break;
            }
            hasher.update(buffer);
            if compress {
                encoder.write_all(buffer)?;
            }
            buffer.len()
        };
        reader.consume(consummed);
    }
    encoder.finish()?;
    Ok(Pack {
        file: tmp_file_path,
        checksum: hasher.finalize().to_hex().to_string(),
    })
}
