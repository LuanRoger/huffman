use std::time::{SystemTime, UNIX_EPOCH};

use huffman::encode_decode_from_file;
use huffman::fs::write_result;

use huffman::compare_result;

fn main() {
    let file_path = "./samples/one/one.txt";
    let output_path_encode = "./samples/one/one_encoded.huff";
    let output_path_decode = "./samples/one/one_decoded.txt";

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let result = encode_decode_from_file(file_path);
    let (encoded_file, decoded_file) =
        write_result(&result, output_path_encode, output_path_decode);
    compare_result(&decoded_file, &encoded_file);

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let duration = end.as_secs_f64() - start.as_secs_f64();
    println!("Execution time: {:.6} seconds", duration);
}
