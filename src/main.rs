use std::{thread};
use std::time::Duration;
use pi_play_lib::segment::{Segment};
use pi_play_lib::temp::{read_temp};
use pi_play_lib::motor::{Motor};
use std::str::FromStr;

const FAHRENHEIT: bool = false;

fn main() {
    let mut segment_display = Segment::new();
    segment_display.init();
    let mut motor = Motor::new();
    motor.stop_motor();
    loop {
        let mut temp = read_temp(FAHRENHEIT);
        println!("temp {}",i32::from_str(&temp).unwrap());
        let motor_on: bool = i32::from_str(&temp).unwrap() > 20000i32;
        if motor_on {
            motor.start_motor();
        } else {
            motor.stop_motor();
        }
        segment_display.display_dec(temp.clone());
        temp.insert(2, '.');
        temp = temp[..5].to_string();
        println!("Current temp: {} {}\n", temp, if FAHRENHEIT {"F"} else {"C"});
        thread::sleep(Duration::from_millis(1000));
    };
}








