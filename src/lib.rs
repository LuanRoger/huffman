#[path = "huffman_lib/mod.rs"]
mod huffman_lib;

pub use huffman_lib::{CompressionResult, encode_decode_from_file, encode_decode_from_string};

pub use huffman_lib::fs;
pub use huffman_lib::tree;

#[path = "utils/mod.rs"]
mod utils;

pub use utils::compare_result;
