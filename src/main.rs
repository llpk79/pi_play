use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();

    let receiver_thread = thread::Builder::new()
        .name("receiver".to_string())
        .spawn(move || loop {
            let (kbytes, seconds, error) = receiver.print_message();
            println!(
                "Message in {} sec\nKB/s {:.3}\n'Error' {:.3}",
                seconds,
                kbytes as f64 / seconds,
                1.0 - error,
            );
        });
    let laser_thread = thread::Builder::new()
        .name("laser".to_string())
        .spawn(move || loop {
            laser.send_message(
                fs::read_to_string("./src/main.rs").unwrap(),
                // "Hello World ".to_string(),
            );
            thread::sleep(Duration::from_millis(250))
        });
    laser_thread.unwrap().join().unwrap();
    receiver_thread.unwrap().join().unwrap();
}
