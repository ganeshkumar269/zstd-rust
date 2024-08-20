mod compress;

pub fn compress(bytes: &[u8]) -> Result<&[u8], &str> {
    Err("TODO")
}

pub fn decompress(bytes: &[u8]) -> Result<&[u8], &str> {
    Err("TODO")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let compressed_data = compress("example_input".as_bytes());
    }

    #[test]
    fn test_decompress() {
        let decompressed_data = decompress("example_input".as_bytes());
    }
}
