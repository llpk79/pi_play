use std::fs::read_to_string;
use std::str::FromStr;

pub fn read_temp(fahrenheit: bool) -> String {
    let file_str: String = read_to_string("/sys/bus/w1/devices/28-3ce1d443e7e1/w1_slave")
        .unwrap()
        .parse()
        .unwrap();
    let mut temp_str: String = file_str[69..74].to_string();
    let mut temp = f32::from_str(&temp_str).unwrap();
    temp /= 1000.0;
    if fahrenheit {
        temp = (temp * 9.0 / 5.0) + 32.0;
    }
    temp_str = temp.to_string() + "00";
    temp_str
}
