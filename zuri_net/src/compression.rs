use std::io::{Read, Write};

#[derive(Debug, Copy, Clone)]
pub enum Compression {
    Deflate,
    Snappy,
}

impl Compression {
    pub fn compress(&self, data: Vec<u8>) -> Result<Vec<u8>, String> {
        match self {
            Compression::Deflate => {
                let mut compressor = libdeflater::Compressor::new(libdeflater::CompressionLvl::default());

                let compress_bound = compressor.deflate_compress_bound(data.len());
                let mut compressed_data = vec![0; compress_bound];

                let actual_size = compressor.deflate_compress(
                    &data,
                    &mut compressed_data,
                )?;

                compressed_data.resize(actual_size, 0);
                Ok(compressed_data)
            }
            Compression::Snappy => {
                let mut encoder = snap::write::FrameEncoder::new(Vec::new());
                encoder.write_all(&data)?;
                Ok(encoder.into_inner()?)
            }
        }
    }

    pub fn decompress(&self, data: Vec<u8>) -> Result<Vec<u8>, String> {
        match self {
            Compression::Deflate => {
                let mut decompressor = libdeflater::Decompressor::new();
                let mut decompressed_data = vec![0; 1024 * 1024 * 3];

                let actual_size = decompressor.deflate_decompress(
                    &data,
                    &mut decompressed_data,
                )?;

                decompressed_data.resize(actual_size, 0);
                Ok(decompressed_data)
            }
            Compression::Snappy => {
                let mut decoder = snap::read::FrameDecoder::new(data);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;
                Ok(decompressed_data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compression::Compression;

    #[test]
    fn test_deflate() {
        let data = b"Hello, world!";
        let compressed = Compression::Deflate.compress(data.into())?;
        let decompressed = Compression::Deflate.decompress(compressed);
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_snappy() {
        let data = b"Hello, world!";
        let compressed = Compression::Snappy.compress(data.into())?;
        let decompressed = Compression::Snappy.decompress(compressed);
        assert_eq!(data, &decompressed[..]);
    }
}
