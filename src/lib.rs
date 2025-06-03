mod compress;

pub fn compress(bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    Err(String::from("TODO"))
}

pub fn decompress(bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    Err(String::from("TODO"))
}

#[cfg(test)]
mod tests {
    use crate::compress::{get_frame_header_as_bytes, Block, BlockType};
    use super::*;

    #[test]
    fn test_compress() {
        let compressed_data = compress("example_input".as_bytes().to_vec());
    }

    #[test]
    fn test_decompress() {
        let decompressed_data = decompress("example_input".as_bytes().to_vec());
    }

    #[test]
    fn sample_test(){
        // let mut a = Block::default();
        // a.set_block_header(false, BlockType::RAW_BLOCK, 2);
        // println!("{:?}", a);
        // println!("{:?}", &1024_i32.to_be_bytes()[0..3].to_vec());
        // println!("{:?}", &1024_i32.to_be_bytes().to_vec()[0..3]);
        println!("{:?}", get_frame_header_as_bytes(3, true, false, 0, 32_u64))
    }
    #[test]
    fn random(){
        println!("{}", (true as u8));
        println!("{}", (6 | 1))
    }

}