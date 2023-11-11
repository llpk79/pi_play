use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();
    let message = fs::read_to_string("./src/temp_humid.rs").expect("error opening file");
    // let message = "Hello.".to_string();

    // Encode message with Huffman coding.
    // Use a binary heap to store the frequency of each character.
    // Not a difficult algorithm. should be easy. go for it.
    let mut heap = BinaryHeap::new();
    let mut freq = HashMap::new();
    for c in message.chars() {
        let count = freq.entry(c).or_insert(0);
        *count += 1;
    }
    for (c, f) in freq {
        heap.push((f, c));
    }
    while heap.len() > 1 {
        let (f1, c1) = heap.pop().unwrap();
        let (f2, c2) = heap.pop().unwrap();
        heap.push((f1 + f2, '%'));
    }
    let mut tree = heap.pop().unwrap();
    println!("{:?}", tree);


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
