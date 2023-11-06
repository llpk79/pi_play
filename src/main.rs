use std::time::Duration;
use std::{fs, thread};
use pi_play_lib::lasers::{Laser, Receiver};

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();

    let laser_thread = thread::spawn(move || loop {
        laser.send_message(
            fs::read_to_string("./src/main.rs").unwrap()
            // "Hello World ".to_string(),
        );
        thread::sleep(Duration::from_secs(2))
    });
    let receiver_thread = thread::spawn(move || loop {
        let start = chrono::Utc::now();
        receiver.print_message();
        let end = chrono::Utc::now();
        println!("Message in {} sec", (end - start).num_seconds());
        thread::sleep(Duration::from_secs(2));
    });
    laser_thread.join().unwrap();
    receiver_thread.join().unwrap();
}
