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

    pub fn new_box(node: Node) -> Box<Node> {
        Box::new(node)
    }
}

impl HuffTree {
    pub fn new() -> HuffTree {
        HuffTree { root: None }
    }

    pub fn build_tree(&mut self, freq_map: HashMap<char, i32>) {
        let mut node_vec: Vec<Box<Node>> = {
            freq_map.iter().map(|(char_, freq)| {
                Node::new_box(Node::new(*freq, Some(*char_)))
            }).collect()
        };
        while node_vec.len() > 2 {
            node_vec.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
            let node1 = node_vec.pop().unwrap();
            let node2 = node_vec.pop().unwrap();
            let mut new_node = Node::new_box(Node::new(node1.freq + node2.freq, None));
            new_node.left = Some(node1);
            new_node.right = Some(node2);
            node_vec.push(new_node);
        }
        self.root = Some(node_vec.pop().unwrap());
    }

    pub fn encode(&mut self, message: String) -> String {
        let mut encoded_message = String::new();
        for char_ in message.chars() {
            encoded_message += &self.encode_char(char_);
        }
        encoded_message
    }

    fn encode_char(&mut self, char_: char) -> String {
        let mut encoded_char = String::new();
        let mut node = self.root.as_ref().unwrap();
        while !Some(&node.left).is_none() && !Some(&node.right).is_none() {
            if node.char_ == Some(char_) {
                encoded_char += "0";
                node = node.left.as_ref().unwrap();
            } else {
                encoded_char += "1";
                node = node.right.as_ref().unwrap();
            }
        }
        encoded_char
    }

    pub fn decode(&mut self, message: Vec<u32>) -> String {
        let mut decoded_message = String::new();
        let mut node = self.root.as_ref().unwrap();
        for bit in message {
            if bit == 0 {
                if let Some(ref left) = &node.left {
                    node = left;
                }
            } else {
                if let Some(ref right) = &node.right {
                    node = right;
                }
            }
            if let Some(ch) = node.char_ {
                decoded_message.push(ch);
                node = self.root.as_ref().unwrap();
            }
        }
        decoded_message
    }
}

