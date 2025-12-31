use std::collections::HashMap;

pub struct Tree {
    pub root: Option<Box<Node>>,
    pub codes: HashMap<u8, String>,
}
#[derive(Eq, Hash, Debug)]
pub struct Node {
    pub freq: usize,
    pub char: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub parent: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: None,
            codes: HashMap::new(),
        }
    }

    pub fn with_root(node: Node) -> Self {
        Tree {
            root: Some(Box::new(node)),
            codes: HashMap::new(),
        }
    }

    pub fn assign_codes(&mut self) {
        let mut code = String::new();
        let mut codes: HashMap<u8, String> = HashMap::new();

        if let Some(ref root) = self.root {
            self.assign_codes_helper(root, &mut codes, &mut code);
        }

        self.codes = codes;
    }

    pub fn encode(&self, text: Vec<u8>) -> Vec<u8> {
        let mut encoded_bits = String::new();

        for byte in text {
            if let Some(code) = self.codes.get(&byte) {
                encoded_bits.push_str(code);
            }
        }

        let mut encoded_bytes: Vec<u8> = Vec::new();
        let mut current_byte: u8 = 0;
        let mut bit_count: u8 = 0;

        for bit in encoded_bits.chars() {
            current_byte <<= 1;
            if bit == '1' {
                current_byte |= 1;
            }
            bit_count += 1;

            if bit_count == 8 {
                encoded_bytes.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }

        if bit_count > 0 {
            current_byte <<= 8 - bit_count;
            encoded_bytes.push(current_byte);
        }

        encoded_bytes
    }

    pub fn decode(&self, code: Vec<u8>) -> String {
        let mut decoded_string = String::new();
        let mut current_node = self.root.as_ref();

        if self.root.is_none() {
            return decoded_string;
        }

        // Traverse the tree bit by bit. Bits are stored MSB-first in each byte.
        for byte in code {
            // iterate from bit 7 (MSB) down to 0 (LSB)
            for bit_index in (0..8).rev() {
                // If current_node is None, we cannot continue decoding
                if current_node.is_none() {
                    break;
                }

                let node_ref = current_node.unwrap();

                let bit_is_one = ((byte >> bit_index) & 1) == 1;

                // Move left on 0, right on 1
                current_node = if bit_is_one {
                    node_ref.right.as_ref()
                } else {
                    node_ref.left.as_ref()
                };

                // If we've reached a leaf node, append the character and reset to root
                if let Some(n) = current_node {
                    if let Some(ch) = n.char {
                        decoded_string.push(ch as char);
                        current_node = self.root.as_ref();
                    }
                } else {
                    // Invalid path (possibly padding bits); stop processing further bits
                    break;
                }
            }
        }

        decoded_string
    }

    fn assign_codes_helper(
        &self,
        node: &Box<Node>,
        codes: &mut HashMap<u8, String>,
        code: &mut String,
    ) {
        if let Some(c) = node.char {
            codes.insert(c, code.clone());
            return;
        }

        if let Some(ref left) = node.left {
            code.push('0');
            self.assign_codes_helper(left, codes, code);
            code.pop();
        }

        if let Some(ref right) = node.right {
            code.push('1');
            self.assign_codes_helper(right, codes, code);
            code.pop();
        }
    }
}

impl Node {
    pub fn new(freq: usize, char: Option<u8>) -> Self {
        Node {
            freq,
            char,
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn set_left(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    pub fn set_right(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }

    pub fn set_parrent(&mut self, node: Node) {
        self.parent = Some(Box::new(node))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char
    }
}

impl PartialEq<u8> for Node {
    fn eq(&self, other: &u8) -> bool {
        match self.char {
            Some(c) => c == *other,
            None => false,
        }
    }
}
