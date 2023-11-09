use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();
    let message = fs::read_to_string("./src/temp_humid.rs").expect("error opening file");

    let receiver_thread = thread::Builder::new()
        .name("receiver".to_string())
        .spawn(move || loop {
            receiver.print_message();
        });

    let laser_thread = thread::Builder::new()
        .name("laser".to_string())
        .spawn(move || loop {
            laser.send_message(message.clone());
            thread::sleep(Duration::from_millis(250))
        });

    laser_thread.unwrap().join().unwrap();
    receiver_thread.unwrap().join().unwrap();
}
