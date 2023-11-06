// use pi_play_lib::segment::Segment;
// use pi_play_lib::temp::read_temp;
use std::{fs, thread};
use std::time::Duration;
// use std::time::Duration;
// use pi_play_lib::distance::Distance;
// use pi_play_lib::lcd::LCD;
// use pi_play_lib::temp_humid::measure_temp_humid;
use pi_play_lib::lasers::{Laser, Receiver};

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();

    let laser_thread = thread::spawn(move || loop {
        laser.send_message(
            // fs::read_to_string("./src/distance.rs").unwrap()
            "Hello World ".to_string()
        );
        thread::sleep(Duration::from_secs(2))
    });
    let receiver_thread = thread::spawn(move || loop {
        receiver.print_message();
        thread::sleep(Duration::from_secs(2))
    });
    laser_thread.join().unwrap();
    receiver_thread.join().unwrap();
}
