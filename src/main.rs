use std::{thread};
use std::time::Duration;
use std::fs::read_to_string;
use std::str::FromStr;
use pi_play_lib::segment::{Segment};

const FAHRENHEIT: bool = false;

fn read_temp(fahrenheit: bool) -> String {
    let file_str: String = read_to_string("/sys/bus/w1/devices/28-3ce1d443e7e1/w1_slave").unwrap().parse().unwrap();
    let mut temp_str: String = file_str[69..74].to_string();
    if fahrenheit {
        let mut temp = f32::from_str(&temp_str).unwrap();
        temp /= 1000.0;
        temp = (temp * 9.0/5.0) + 32.0;
        temp_str = temp.to_string().replace(".", "") + "00";
    }
    temp_str
}

fn main() {
    let mut segment_display = Segment::new();
    segment_display.init();
    loop {
        let temp = read_temp(FAHRENHEIT);
        segment_display.display_dec(temp.clone());
        println!("Current temp: {} {}\n", temp, if FAHRENHEIT {"F"} else {"C"});
        thread::sleep(Duration::from_millis(1000));
    };
}








