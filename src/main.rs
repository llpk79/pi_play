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

    let mut code_map: HashMap<String, String> = HashMap::new();
    let mut code = String::new();
    let mut code_stack: Vec<String> = Vec::new();
    let mut char_stack: Vec<String> = Vec::new();
    let (count, chars) = heap.pop().unwrap();
    code_stack.push(code);
    char_stack.push(chars);
    while !char_stack.is_empty() {
        let chars = char_stack.pop().unwrap();
        let code = code_stack.pop().unwrap();
        if chars.len() == 1 {
            code_map.insert(chars, code);
        } else {
            let mut chars = chars.split(" ").collect::<Vec<_>>();
            let char1 = chars.pop().unwrap();
            let char2 = chars.pop().unwrap();
            let code1 = code.clone() + "0";
            let code2 = code + "1";
            code_stack.push(code1);
            code_stack.push(code2);
            char_stack.push(char1.to_string());
            char_stack.push(char2.to_string());
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
