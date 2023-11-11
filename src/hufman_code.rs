// Huffman encoding and decoding.

use ::std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    freq: i32,
    char_: Option<char>,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
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
            freq_map
                .iter()
                .map(|(char_, freq)| Node::new_box(Node::new(*freq, Some(*char_))))
                .collect()
        };
        while node_vec.len() > 1 {
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

    pub fn encode_string(&mut self, string: &mut String) -> Vec<u32> {
        let mut encoded_message = Vec::new();
        let mut code_map = HashMap::new();
        assign_codes(&self.root.as_ref().unwrap(), &mut code_map, "".to_string());
        println!("code_map: {:?}", code_map);
        for char in string.chars() {
            let code = code_map.get(&char).unwrap();
            for bit in code.chars() {
                encoded_message.push(bit.to_digit(10).unwrap());
            }
        }
        encoded_message
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

pub fn assign_codes(tree: &Box<Node>, code_map: &mut HashMap<char, String>, string: String) {
    if let Some(ch) = tree.char_ {
        code_map.insert(ch, "".to_string());
    } else {
        if let Some(l) = &tree.left {
            assign_codes(l, code_map, string.clone() + "0");
        }
        if let Some(r) = &tree.right {
            assign_codes(r, code_map, string.clone() + "1");
        }
    }
}
