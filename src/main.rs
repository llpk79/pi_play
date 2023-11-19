use pi_play_lib::huffman_code::HuffTree;
use pi_play_lib::lasers::{Laser, Receiver};
use pi_play_lib::lcd::LCD;
use pi_play_lib::temp::read_temp;
use std::time::Duration;
use std::{thread};

/// Send a message with a laser!
fn do_laser() {
    // Dummy message to encode temperature stuff.
    let message = "FC::1234567890..        \n".to_string();
    // Compress message with Huffman Coding.
    let mut huff_tree = HuffTree::new();
    huff_tree.build_tree(&message);

    // Pass huff_tree to receiver to decode message.
    let mut receiver = Receiver::new(huff_tree.clone());
    let mut laser = Laser::new(huff_tree);
    let mut lcd = LCD::new();
    lcd.display_init();

    // Start a thread each for the laser and receiver.
    let receiver_thread = thread::Builder::new()
        .name("receiver".to_string())
        .spawn(move || loop {
            let message = receiver.receive_message();
            lcd.display_data(message);
        });

    let laser_thread = thread::Builder::new()
        .name("laser".to_string())
        .spawn(move || loop {
            let celsius = read_temp(false);
            let fahrenheit = read_temp(true);
            let message = format!("C: {celsius}\nF: {fahrenheit}");
            laser.send_message(message);
            thread::sleep(Duration::from_millis(1000))
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
    // let message = fs::read_to_string("./src/huffman_code.rs").expect("File should exist");
    do_laser()
}
