use pi_play_lib::barometer::{Barometer, Mode::{HighRes}};
use pi_play_lib::huffman_code::HuffTree;
use pi_play_lib::lasers::{Laser, Receiver};
use pi_play_lib::lcd::LCD;
use std::thread;
use std::time::Duration;

/// Send a message with a laser!
fn do_laser() {
    // Dummy message to encode temperature stuff.
    let message = "FCBA111222333444555666777888999000....-        \n".to_string();
    // Compress message with Huffman Coding.
    let mut huff_tree = HuffTree::new();
    huff_tree.build_tree(&message);

    // Pass huff_tree to receiver to decode message.
    let mut receiver = Receiver::new(huff_tree.clone());
    let mut laser = Laser::new(huff_tree);

    let mut lcd = LCD::new();
    lcd.display_init();

    let mut barometer = Barometer::new();
    barometer.init();

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

            let raw_c = barometer.read_raw_temp();
            let celsius = barometer.read_temperature(raw_c);
            let fahrenheit = ((celsius as f32 / 10_f32)  * 9.0_f32 / 5.0) + 32.0;

            let mode = HighRes;
            let raw_baro = barometer.read_raw_pressure(&mode);
            let baro = barometer.read_pressure(raw_baro, &mode);

            let altitude = barometer.read_altitude(&mode);

            let message = format!(
                "C {:.1} F {:.1}        \nB {:.1} A {:.1}        ",
                celsius as f32 / 10_f32,
                fahrenheit,
                baro as f32 / 100_f32,
                altitude
            );
            laser.send_message(message);
            thread::sleep(Duration::from_millis(500))
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
