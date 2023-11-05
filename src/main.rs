// use pi_play_lib::segment::Segment;
// use pi_play_lib::temp::read_temp;
use std::thread;
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
            "Type any arbitrary thing, no worries about how long it is, just go for it! It should print out if you hold the laser steady on the receiver for long enough. It will reset itself without error if it's shaky or gets moved.".to_string(),
        );
    });
    let receiver_thread = thread::spawn(move || loop {
        receiver.print_message();
    });
    laser_thread.join().unwrap();
    receiver_thread.join().unwrap();
}
