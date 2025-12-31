use std::fs::File;

pub fn compare_result(source_file: &File, result_file: &File) {
    let source_file_metadata = source_file.metadata();
    let output_file_metadata = result_file.metadata();

    match (source_file_metadata, output_file_metadata) {
        (Ok(source_meta), Ok(output_meta)) => {
            let source_size = source_meta.len();
            let output_size = output_meta.len();

            let diference = source_size - output_size;
            let reduction_percentage = (diference as f64 / source_size as f64) * 100.0;

            println!("Source file size: {} bytes", source_size);
            println!("Encoded file size: {} bytes", output_size);
            println!(
                "Size reduction: {} bytes ({:.2}%)",
                diference, reduction_percentage
            );
        }
        _ => {
            println!("Failed to retrieve file metadata.");
        }
    }
}
