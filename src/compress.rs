
struct FrameHeader{
    frame_header_descriptor: u8,
    window_descriptor: u8, // optional
    dictionary_id: u32, // optional
    frame_content_size: u64
}

pub fn get_frame_header_as_bytes(
    frame_content_size_flag: u8,
    single_segment_flag: bool,
    content_checksum_flag: bool,
    dictionary_id_flag: u8,
    frame_content_size: u64
) -> Vec<u8>{
    let frame_header_descriptor = (
        (
            (
                (
                    (frame_content_size_flag << 1)
                    | (single_segment_flag as u8)
                ) << 3
            ) | (content_checksum_flag as u8)
        ) << 2
    ) | dictionary_id_flag;

    let mut res = Vec::from(frame_header_descriptor.to_le_bytes());
    res.append(&mut Vec::from(frame_content_size.to_le_bytes()));
    res
}


#[derive(Default, Debug)]
pub struct Block {
    block_header:  u32,
    block_content: Vec<u8>,
}

pub enum BlockType {
    RAW_BLOCK,
    RLE_BLOCK,
    COMPRESSED_BLOCK,
    RESERVED
}

impl BlockType {
    fn value(self) -> u8 {
        match (self) {
            BlockType::RAW_BLOCK => 0,
            BlockType::RLE_BLOCK => 1,
            BlockType::COMPRESSED_BLOCK => 2,
            BlockType::RESERVED => 3
        }
    }
}


impl Block {
    pub fn set_block_header(&mut self, is_last_block: bool, block_type: BlockType, block_size: u32) {
        self.block_header = ((block_size << 3) | (block_type.value() as u32)) | (is_last_block as u32);
    }

    fn block_header_as_bytes(self) -> Vec<u8> {
        self.block_header.to_le_bytes()[0..3].to_vec()
    }


}

struct Frame {
    magic_numer: u32, // const to 0xFD2FB528
    frame_header: FrameHeader,
    data_block: Block,
    content_checksum: u32
}


pub fn compress(bytes: Vec<u8>) -> Result<Vec<u8>, String>{
    Err(String::from("TODO"))
}