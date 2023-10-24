use std::{thread};
use std::time::Duration;
use std::fs::read_to_string;
use std::str::FromStr;
use pi_play_lib::segment::{Segment};

fn read_temp(f: bool) -> String {
    let file_str: String = read_to_string("/sys/bus/w1/devices/28-3ce1d443e7e1/w1_slave").unwrap().parse().unwrap();
    println!("file_str {}", file_str);
    let mut temp_str: String = file_str[69..74].to_string();
    if f {
        let mut temp = f32::from_str(&temp_str).unwrap();
        temp /= 1000.0;
        temp = (temp * (9.0/5.0)) + 32.0;
        temp_str = temp.to_string()
    }
    temp_str
}

fn main() {
    let mut segment_display = Segment::new();
    segment_display.init();
    print!("segment {:?}", segment_display);
    loop {
        let temp = read_temp(true);
        segment_display.display_dec(temp.clone());
        println!("Current temp: {} C", temp);
        thread::sleep(Duration::from_millis(1000));
    };
}








