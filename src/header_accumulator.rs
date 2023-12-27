use std::borrow::Cow;
use embed_file::embed_string;

pub const EPOCH_SIZE: u64 = 8192;
const DATA: Cow<str> = embed_string!("assets/acc_values.txt");

pub fn get_epoch(block_number: u64) -> u64 {
    block_number / EPOCH_SIZE
}

pub fn get_value_for_block(block_number: u64) -> Vec<u8> {
    let epoch = get_epoch(block_number);
    let line = DATA.lines().nth(epoch as usize).unwrap();
    hex::decode(line).unwrap()
}