use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};
use std::collections::HashMap;

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();
    let message = fs::read_to_string("./src/temp_humid.rs").expect("error opening file");
    // let message = "Hello.".to_string();
    // Create huffman encoding for message.
    let mut freq_map = HashMap::new();
    for char in message.chars() {
        let count = freq_map.entry(char).or_insert(0);
        *count += 1;
    }
    let mut freq_vec: Vec<(char, i32)> = Vec::new();
    for (char, count) in freq_map {
        freq_vec.push((char, count));
    }
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let mut huffman_map = HashMap::new();
    let mut huffman_vec: Vec<(char, String)> = Vec::new();
    for (char, count) in freq_vec {
        huffman_map.insert(char, count);
        huffman_vec.push((char, "".to_string()));
    }
    while huffman_vec.len() > 1 {
        let (char1, code1) = huffman_vec.pop().unwrap();
        let (char2, code2) = huffman_vec.pop().unwrap();
        for (char, code) in &mut huffman_vec {
            if code1.contains(*char) {
                *code = "0".to_string() + code;
            } else if code2.contains(*char) {
                *code = "1".to_string() + code;
            }
        }
        huffman_vec.push((char1, "0".to_string() + &code1));
        huffman_vec.push((char2, "1".to_string() + &code2));
        huffman_vec.sort_by(|a, b| b.1.cmp(&a.1));
    }
    let mut huffman_code_map = HashMap::new();
    for (char, code) in huffman_vec {
        huffman_code_map.insert(char, code);
    }
    println!("huff {:?}", huffman_code_map);
    let mut encoded_message = String::new();
    for char in message.chars() {
        encoded_message = encoded_message + &huffman_code_map[&char];
    }

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
