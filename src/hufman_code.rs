// Huffman encoding and decoding.

use::std::collections::HashMap;

pub struct Node {
    freq: i32,
    char_: Option<char>,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

pub struct HuffTree {
    root: Option<Box<Node>>,
}

impl Node {
    pub fn new(freq: i32, char_: Option<char>) -> Node {
        Node {
            freq,
            char_,
            right: None,
            left: None,
        }
    }
}

impl HuffTree {
    pub fn new() -> HuffTree {
        HuffTree { root: None }
    }

    pub fn build_tree(&mut self, freq_map: HashMap<char, i32>) {
        let mut node_vec: Vec<Node> = Vec::new();
        for (char_, freq) in freq_map {
            node_vec.push(Node::new(freq, Some(char_)));
        }
        node_vec.sort_by(|a, b| a.freq.cmp(&b.freq));
        while node_vec.len() > 1 {
            let mut node1 = node_vec.remove(0);
            let mut node2 = node_vec.remove(0);
            let mut new_node = Node::new(node1.freq + node2.freq, None);
            new_node.left = Some(Box::new(node1));
            new_node.right = Some(Box::new(node2));
            node_vec.push(new_node);
            node_vec.sort_by(|a, b| a.freq.cmp(&b.freq));
        }
        self.root = Some(Box::new(node_vec.remove(0)));
    }

    pub fn encode(&self, message: String) -> String {
        let mut encoded_message = String::new();
        for char_ in message.chars() {
            encoded_message += &self.encode_char(char_);
        }
        encoded_message
    }

    fn encode_char(&self, char_: char) -> String {
        let mut encoded_char = String::new();
        let mut node = self.root.as_ref().unwrap();
        while node.char_.is_none() {
            if node.left.as_ref().unwrap().char_.unwrap() == char_ {
                encoded_char += "0";
                node = node.left.as_ref().unwrap();
            } else {
                encoded_char += "1";
                node = node.right.as_ref().unwrap();
            }
        }
        encoded_char
    }

    pub fn decode(&self, message: Vec<u32>) -> String {
        let mut decoded_message = String::new();
        let mut node = self.root.as_ref().unwrap();
        for bit in message {
            if bit == 0 {
                node = node.left.as_ref().unwrap();
            } else {
                node = node.right.as_ref().unwrap();
            }
            if node.char_.is_some() {
                decoded_message += &node.char_.unwrap().to_string();
                node = self.root.as_ref().unwrap();
            }
        }
        decoded_message
    }
}

