use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();
    let message = fs::read_to_string("./src/temp_humid.rs").expect("error opening file");
    // let message = "Hello.".to_string();
    // Create huffman encoding for message utilizing BinaryHeap.
    let mut heap = BinaryHeap::new();
    let mut freq_map = HashMap::new();
    for char in message.chars() {
        let count = freq_map.entry(char).or_insert(0);
        *count += 1;
    }
    for (char, freq) in freq_map {
        heap.push((freq, char));
    }
    while heap.len() > 1 {
        let (freq1, _) = heap.pop().unwrap();
        let (freq2, _) = heap.pop().unwrap();
        heap.push((freq1 + freq2, '%'));
    }

    // traverse tree to create encoding map.
    fn traverse_tree(
        node: &mut BinaryHeap<(i32, char)>,
        encoding_map: &mut HashMap<char, String>,
        encoding: String,
    ) {
        if let Some((_, char)) = node.peek() {
            if char != &'%' {
                encoding_map.insert(*char, encoding.clone());
            }
        }
        if let Some((_, char)) = node.pop() {
            traverse_tree(node, encoding_map, encoding.clone() + "0");
            traverse_tree(node, encoding_map, encoding + "1");
        }
    }

    let mut encoding_map = HashMap::new();
    traverse_tree(&mut heap, &mut encoding_map, "".to_string());

    // encode message.
    let mut encoded_message = String::new();
    for char in message.chars() {
        encoded_message += &encoding_map[&char];
    }
    println!("encoded message {}", encoded_message);

    let receiver_thread = thread::Builder::new()
        .name("receiver".to_string())
        .spawn(move || loop {
            receiver.print_message();
        });

    let laser_thread = thread::Builder::new()
        .name("laser".to_string())
        .spawn(move || loop {
            laser.send_message(message.clone());
            thread::sleep(Duration::from_millis(2500))
        });

    laser_thread.unwrap().join().unwrap();
    receiver_thread.unwrap().join().unwrap();
}
