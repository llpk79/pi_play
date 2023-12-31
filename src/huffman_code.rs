// Huffman encoding and decoding.
// More or less stolen from https://github.com/pcein/rust-for-fun/blob/master/huffman-coding/tree.rs

use ::std::collections::HashMap;

struct Node {
    freq: i32,
    char_: Option<char>,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

pub struct HuffTree {
    root: Option<Box<Node>>,
}

impl Node {
    fn new(freq: i32, char_: Option<char>) -> Node {
        Node {
            freq,
            char_,
            right: None,
            left: None,
        }
    }

    fn new_box(node: Node) -> Box<Node> {
        Box::new(node)
    }
}

impl HuffTree {
    pub fn new() -> HuffTree {
        HuffTree { root: None }
    }

    /// Map characters in message to their frequency in message.
    fn create_frequency_map(&mut self, message: &String) -> HashMap<char, i32> {
        let mut frequency_map = HashMap::new();
        for char in message.chars() {
            let count = frequency_map.entry(char).or_insert(0);
            *count += 1;
        }
        frequency_map
    }

    /// Create HuffmanTree to code characters with greater frequency with a short codes and
    /// infrequent characters with long codes.
    fn build_tree(&mut self, message: &String) {
        // Build a vec of single node HuffTrees from the frequency map.
        let frequency_map = self.create_frequency_map(message);
        let mut node_vec: Vec<Box<Node>> = {
            frequency_map
                .iter()
                .map(|(char_, freq)| Node::new_box(Node::new(*freq, Some(*char_))))
                .collect()
        };
        // Pop the top two nodes, combine their frequencies to create a new Node with char = None.
        // Assign the larger popped node as the new node's left, the smaller as right.
        // Keep doing this until len is 1. This is the root of the sorted HuffTree.
        while node_vec.len() > 1 {
            node_vec.sort_by(|a, b| (&b.freq).cmp(&a.freq));
            let node1 = node_vec.pop().expect("Vec should have elements.");
            let node2 = node_vec.pop().expect("Vec should have elements.");
            let mut new_node = Node::new_box(Node::new(node1.freq + node2.freq, None));
            new_node.left = Some(node1);
            new_node.right = Some(node2);
            node_vec.push(new_node);
        }
        self.root = Some(node_vec.pop().expect("Tree must have root."));
    }

    /// Map characters to their codes by traversing HuffTree.
    ///
    /// Recurse to leaf nodes where characters reside.
    ///
    /// Append to string each step down the path to the char.
    ///
    /// A move to the left appends a '0', to the right a '1'.
    fn assign_codes(&self, tree: &Box<Node>, code_map: &mut HashMap<char, String>, string: String) {
        if let Some(char) = &tree.char_ {
            code_map.insert(*char, string);
        } else {
            if let Some(left) = &tree.left {
                self.assign_codes(left, code_map, string.clone() + "0");
            }
            if let Some(right) = &tree.right {
                self.assign_codes(right, code_map, string + "1");
            }
        }
    }

    /// Use char_code_map populated by assign_codes to map characters their to binary codes.
    ///
    /// Create checksum as vec is built. Append 32 bit checksum to message vec.
    fn encode_string(&mut self, message: &String) -> Vec<u32> {
        let mut encoded_message = Vec::new();
        let mut char_code_map = HashMap::new();
        self.assign_codes(
            &self.root.as_ref().expect("tree exists"),
            &mut char_code_map,
            "".to_string(),
        );
        let mut checksum = 0_u32;
        let mut byte_index = 0_u8;
        for char in message.chars() {
            let code = char_code_map.get(&char).expect("All message chars in map.");
            for bit in code.chars() {
                let bit = bit.to_digit(10).expect("Bits must be digits");
                encoded_message.push(bit);
                checksum += bit << byte_index;
                match byte_index {
                    7 => byte_index = 0,
                    _ => byte_index += 1,
                }
            }
        }
        let mut check_vec = Vec::new();
        for bit in (0..32).map(|n| (checksum >> n) & 1) {
            check_vec.push(bit);
        }
        Vec::from([encoded_message, check_vec].concat())
    }

    /// Build the tree and encode the message.
    pub fn encode(&mut self, message: String) -> Vec<u32> {
        self.build_tree(&message);
        self.encode_string(&message)
    }

    /// Use encoded message to traverse tree and find characters.
    ///
    /// A '0' moves down the tree to the left, '1' to the right.
    ///
    /// Only leaf nodes have characters so if we found one that's it.
    pub fn decode(&self, encoded_message: Vec<u32>) -> String {
        let mut decoded_message = String::new();
        let mut node = self.root.as_ref().expect("Tree must have root.");
        for bit in encoded_message {
            if bit == 0 {
                if let Some(ref left) = &node.left {
                    node = left;
                }
            } else {
                if let Some(ref right) = &node.right {
                    node = right;
                }
            }
            if let Some(char) = node.char_ {
                decoded_message.push(char);
                node = self.root.as_ref().expect("Tree must have root.");
            }
        }
        decoded_message
    }
}
