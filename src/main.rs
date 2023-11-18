use pi_play_lib::huffman_code::HuffTree;
use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};

/// Send a message with a laser!
pub fn do_laser(message: String) {
    // Compress message with Huffman Coding.
    let mut huff_tree = HuffTree::new();
    let encoded_message = huff_tree.encode(message);

    // Pass huff_tree to receiver to decode message.
    let mut receiver = Receiver::new(huff_tree);
    let mut laser = Laser::new(encoded_message);

    // Start a thread each for the laser and receiver.
    let receiver_thread = thread::Builder::new()
        .name("receiver".to_string())
        .spawn(move || loop {
            receiver.print_message();
        });

    let laser_thread = thread::Builder::new()
        .name("laser".to_string())
        .spawn(move || loop {
            laser.send_message();
            thread::sleep(Duration::from_millis(2000))
        });

    receiver_thread
        .expect("Thread should exist")
        .join()
        .expect("Thread should close");
    laser_thread
        .expect("Thread should exist")
        .join()
        .expect("Thread should close");
}

fn main() {
    let message = fs::read_to_string("./src/main.rs").expect("File should exist");
    // let message = "Hello World.".to_string();

    do_laser(message)
}
