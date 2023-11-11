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
    let mut encoding_map = HashMap::new();

    // traverse tree to create encoding map.
    let heap_vec = heap.into_vec();
    let mut tree = heap_vec[0].1.to_string();
    let mut encoding = String::new();
    while tree.len() > 0 {
        let char = tree.remove(0);
        if char == '%' {
            encoding.pop();
            let char = tree.remove(0);
            encoding_map.insert(char, encoding.clone());
            encoding.pop();
        } else {
            encoding.push('0');
            tree.push(char);
            tree.push('1');
        }
    }
    println!("{:?}", encoding_map);


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
