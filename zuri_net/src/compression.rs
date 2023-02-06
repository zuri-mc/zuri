use std::io::{Read, Write};

use bytes::Buf;
use zuri_net_derive::packet;

#[packet(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Compression {
    Deflate,
    Snappy,
}

impl Compression {
    pub fn compress(&self, data: &mut Vec<u8>) -> Result<(), String> {
        let temp_input = data.clone();
        match self {
            Compression::Deflate => {
                let mut compressor = libdeflater::Compressor::new(libdeflater::CompressionLvl::default());

                let compress_bound = compressor.deflate_compress_bound(data.len());
                data.resize(compress_bound, 0);

                let actual_size = compressor.deflate_compress(
                    &temp_input,
                    data,
                ).map_err(|e| format!("failed to compress data: {}", e))?;

                data.resize(actual_size, 0);
                Ok(())
            }
            Compression::Snappy => {
                let mut encoder = snap::write::FrameEncoder::new(data);
                encoder.write_all(&temp_input).map_err(|e| format!("failed to compress data: {}", e))?;
                Ok(())
            }
        }
    }

    pub fn decompress(&self, data: &mut Vec<u8>) -> Result<(), String> {
        let temp_input = data.clone();
        match self {
            Compression::Deflate => {
                data.resize(1024 * 1024 * 4, 0);

                let mut decompressor = libdeflater::Decompressor::new();
                let actual_size = decompressor.deflate_decompress(
                    &temp_input,
                    data,
                ).map_err(|e| format!("failed to decompress data: {}", e))?;

                data.resize(actual_size, 0);
                Ok(())
            }
            Compression::Snappy => {
                let mut decoder = snap::read::FrameDecoder::new(temp_input.reader());
                let size = decoder.read_to_end(data).map_err(|e| format!("failed to decompress data: {}", e))?;
                data.resize(size, 0);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compression::Compression;

    #[test]
    fn test_deflate() {
        let data = b"Hello, world!".to_vec();
        let mut processed_data: Vec<u8> = data.clone().to_vec();
        Compression::Deflate.compress(&mut processed_data).unwrap();
        Compression::Deflate.decompress(&mut processed_data).unwrap();
        assert_eq!(data, processed_data);
    }

    #[test]
    fn test_snappy() {
        let data = b"Hello, world!".to_vec();
        let mut processed_data: Vec<u8> = data.clone().to_vec();
        Compression::Snappy.compress(&mut processed_data).unwrap();
        Compression::Snappy.decompress(&mut processed_data).unwrap();
        assert_eq!(data, processed_data);
    }
}
