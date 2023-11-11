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
    let mut char_counts: HashMap<String, i32> = HashMap::new();
    for char in message.chars() {
        let count = char_counts.entry(format!("{}", char)).or_insert(0);
        *count += 1;
    }
    for (char, count) in char_counts {
        heap.push((count, char));
    }
    while heap.len() > 1 {
        let (count1, char1) = heap.pop().unwrap();
        let (count2, char2) = heap.pop().unwrap();
        heap.push((count1 + count2, char1 + &format!(" {}", char2)));
    }
    let mut code_map: HashMap<char, String> = HashMap::new();
    let mut code = String::new();
    let mut char_stack = Vec::new();
    let (count, chars) = heap.pop().unwrap();
    for char in chars.split(" ") {
        char_stack.push(char);
    }
    while !char_stack.is_empty() {
        let char = char_stack.pop().unwrap();
        if char.len() == 1 {
            code_map.insert(char.chars().next().unwrap(), code.clone());
            code.pop();
        } else {
            code.push('0');
            char_stack.push(&char[1..]);
            code.push('1');
            char_stack.push(&char[0..1]);
        }
    }
    println!("{:?}", code_map);

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
