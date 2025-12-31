pub mod fs;
pub mod tree;

use std::fs::read;
use tree::{Node, Tree};

use priority_queue::PriorityQueue;

pub struct CompressionResult {
    pub compressed_data: Vec<u8>,
    pub original_data: String,
}

impl CompressionResult {
    pub fn new(compressed_data: Vec<u8>, original_data: String) -> Self {
        CompressionResult {
            compressed_data,
            original_data,
        }
    }
}

fn build_tree_from_bytes(bytes: &[u8]) -> Tree {
    let mut queue: PriorityQueue<u8, usize> = PriorityQueue::new();

    for byte in bytes {
        match queue.get(byte) {
            Some((_, priority)) => {
                queue.change_priority(byte, priority + 1);
            }
            None => {
                queue.push(*byte, 1);
            }
        }
    }

    let mut nodes: Vec<Node> = Vec::new();
    for (byte, freq) in queue.into_iter() {
        nodes.push(Node::new(freq, Some(byte)));
    }

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| a.freq.cmp(&b.freq));
        let left = nodes.remove(0);
        let right = nodes.remove(0);
        let mut parent = Node::new(left.freq + right.freq, None);
        parent.set_left(left);
        parent.set_right(right);
        nodes.push(parent);
    }

    let mut tree = Tree::with_root(nodes.remove(0));
    tree.assign_codes();
    tree
}

pub fn encode_decode_from_string(input: String) -> CompressionResult {
    let tree = build_tree_from_bytes(input.as_bytes());

    let encoded = tree.encode(input.as_bytes().to_vec());

    let to_decode = encoded.clone();
    let decode = tree.decode(to_decode);

    CompressionResult::new(encoded, decode)
}

pub fn encode_decode_from_file(file_path: &str) -> CompressionResult {
    let file_content = read(file_path).unwrap();

    let tree = build_tree_from_bytes(&file_content);

    let encoded = tree.encode(file_content);

    let to_decode = encoded.clone();
    let decode = tree.decode(to_decode);

    CompressionResult::new(encoded, decode)
}
