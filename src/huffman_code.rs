// Huffman encoding and decoding.
// More or less stolen from https://github.com/pcein/rust-for-fun/blob/master/huffman-coding/tree.rs

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

    /// Create HuffmanTree to code characters with greater frequency with a shorter code
    /// longer codes for infrequent characters.
    pub fn build_tree(&mut self, freq_map: HashMap<char, i32>) {
        // Build a vec of single node HuffTrees from the frequency map.
        let mut node_vec: Vec<Box<Node>> = {
            freq_map
                .iter()
                .map(|(char_, freq)| Node::new_box(Node::new(*freq, Some(*char_))))
                .collect()
        };
        // Pop the top two nodes, combine their frequencies to create a new Node with char = None.
        // Assign the larger popped node as the new node's left, the smaller as right.
        // Keep doing this until len is 1, this is the root of sorted HuffTree.
        while node_vec.len() > 1 {
            node_vec.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
            let node1 = node_vec.pop().expect("Vec has elements.");
            let node2 = node_vec.pop().expect("Vec has elements.");
            let mut new_node = Node::new_box(Node::new(node1.freq + node2.freq, None));
            new_node.left = Some(node1);
            new_node.right = Some(node2);
            node_vec.push(new_node);
        }
        node_vec.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        self.root = Some(node_vec.pop().expect("Tree must have root."));
    }

    /// Map characters to their codes by traversing HuffTree.
    /// Recurse to leaf nodes where characters reside.
    /// Append path to char as the code.
    /// A move to the left appends a '0', to the right a '1'.
    fn assign_codes(&mut self, tree: &Box<Node>, code_map: &mut HashMap<char, String>, string: String) {
        if let Some(char) = &tree.char_ {
            code_map.insert(*char, string);
        } else {
            if let Some(left) = &tree.left {
                self.assign_codes(left, code_map, string.clone() + "0");
            }
            if let Some(right) = &tree.right {
                self.assign_codes(right, code_map, string.clone() + "1");
            }
        }
    }
    /// Use code_map created by assign_codes to map characters to binary codes.
    /// Create checksum as vec is built. Append 32 bit sum to vec.
    pub fn encode_string(&mut self, string: &mut String) -> Vec<u32> {
        let mut encoded_message = Vec::new();
        let mut code_map = HashMap::new();
        self.assign_codes(
            &self.root.clone().expect("tree exists"),
            &mut code_map,
            "".to_string(),
        );
        let mut checksum = 0_u32;
        let mut byte_index = 1_u8;
        for char in string.chars() {
            let code = code_map.get(&char).expect("All message chars in map.");
            for bit in code.chars() {
                let bit = bit.to_digit(10).expect("Bits must be digits");
                encoded_message.push(bit);
                checksum += bit << byte_index;
                match byte_index {
                    8 => byte_index = 1,
                    _ => byte_index += 1,
                }
            }
        }
        let mut check_vec = Vec::new();
        for bit in (1..=32).map(|n| (checksum >> n) & 1) {
            check_vec.push(bit as u32);
        }
        Vec::from([encoded_message, check_vec].concat())
    }

    /// Use code to traverse tree to find characters.
    /// A '0' moves down the tree to the left, '1' to the right.
    /// Only leaf nodes have characters so if we found one that's it.
    pub fn decode(&mut self, message: Vec<u32>) -> String {
        let mut decoded_message = String::new();
        let mut node = self.root.as_ref().expect("Tree must have root.");
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
                node = self.root.as_ref().expect("Tree must have root.");
            }
        }
        decoded_message
    }
}

