use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};
use std::collections::HashMap;

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();
    let message = fs::read_to_string("./src/temp_humid.rs").expect("error opening file");
    // let message = "Hello.".to_string();
    // count the unicode characters in the message
    let mut char_map: HashMap<char, u32> = HashMap::new();
    for char in message.chars() {
        let count = char_map.entry(char).or_insert(0);
        *count += 1;
    }
    println!("char_map {:?}", char_map);

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
