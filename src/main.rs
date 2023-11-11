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
    // Produce a hashmap of codes to chars for decoding.
    let mut heap = BinaryHeap::new();
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    for char in message.chars() {
        let count = char_counts.entry(char).or_insert(0);
        *count += 1;
    }
    for (char, count) in char_counts {
        heap.push((count, char));
    }
    while heap.len() > 1 {
        let (count1, char1) = heap.pop().unwrap();
        let (count2, char2) = heap.pop().unwrap();
        heap.push((count1 + count2, char1));
        heap.push((count1 + count2, char2));
    }
    let mut code_map: HashMap<char, String> = HashMap::new();
    let mut code = String::new();
    let mut code_stack: Vec<(i32, char)> = Vec::new();
    let mut code_stack2: Vec<(i32, char)> = Vec::new();

    while let Some((count, char)) = heap.pop() {
        code_stack.push((count, char));
    }
    while let Some((count, char)) = code_stack.pop() {
        code_stack2.push((count, char));
    }
    while let Some((count, char)) = code_stack2.pop() {
        if char.is_alphanumeric() {
            code_map.insert(char, code.clone());
        }
        code.push('0');
        if count < 0 {
            code.pop();
            code.push('1');
        }
    }
    println!("{:?}", code_map);
    // Send message.
    let mut encoded_message = String::new();
    for char in message.chars() {
        encoded_message.push_str(&code_map[&char]);
    }
    println!("{}", encoded_message);


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
