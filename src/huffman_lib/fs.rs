use std::fs::{File, write};

use crate::huffman_lib::CompressionResult;

pub fn write_result(
    compression_result: &CompressionResult,
    output_path_encode: &str,
    output_path_decode: &str,
) -> (File, File) {
    if let Ok(_) = write(output_path_encode, &compression_result.compressed_data) {
        println!("File encoded and saved to {}", output_path_encode);
    }
    if let Ok(_) = write(output_path_decode, &compression_result.original_data) {
        println!("File decoded and saved to {}", output_path_decode);
    }

    let enconded_file = File::open(output_path_encode).unwrap();
    let decoded_file = File::open(output_path_decode).unwrap();

    (enconded_file, decoded_file)
}
