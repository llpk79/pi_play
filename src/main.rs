use pi_play_lib::lasers::{Laser, Receiver};
use pi_play_lib::hufman_code::HuffTree;
use std::time::Duration;
use std::{fs, thread};
use std::collections::HashMap;


fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();
    // let message = fs::read_to_string("./src/lib.rs").expect("error opening file");
    let mut message = "Hello World.".to_string();
    let mut freq_map = HashMap::new();
    for char in message.chars() {
        let cout = freq_map.entry(char).or_insert(0);
        *cout += 1;
    }
    println!("freq_map: {:?}", freq_map);
    let mut huff_tree = HuffTree::new();
    huff_tree.build_tree(freq_map);
    let encoded_message = huff_tree.encode_string(&mut message);

    let receiver_thread = thread::Builder::new()
        .name("receiver".to_string())
        .spawn(move || loop {
            receiver.print_message(&mut huff_tree);
        });

    let laser_thread = thread::Builder::new()
        .name("laser".to_string())
        .spawn(move || loop {
            laser.send_message(encoded_message.clone());
            thread::sleep(Duration::from_millis(2500))
        });

    laser_thread.unwrap().join().unwrap();
    receiver_thread.unwrap().join().unwrap();
}
