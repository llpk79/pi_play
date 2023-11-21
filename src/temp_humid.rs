use std::str::FromStr;
use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::thread;
use std::time::Duration;

const PIN: u16 = 25;

pub fn measure_temp_humid() -> (f32, f32) {
    let mut data = Vec::new();
    let mut start_pin = gpio::sysfs::SysFsGpioOutput::open(PIN).unwrap();
    start_pin.set_value(false).unwrap();
    thread::sleep(Duration::from_micros(20));
    start_pin.set_value(true).unwrap();
    let mut data_pin = gpio::sysfs::SysFsGpioInput::open(PIN).unwrap();
    while data_pin.read_value().unwrap() == Low {
        continue;
    }
    while data_pin.read_value().unwrap() == High {
        continue;
    }
    while data.len() < 40 {
        while data_pin.read_value().unwrap() == Low {
            continue;
        }
        let start = chrono::Utc::now();
        while data_pin.read_value().unwrap() == High {
            continue;
        }
        let end = chrono::Utc::now();
        let bit_time = end - start;
        println!("bit time {:?}", bit_time.num_microseconds().unwrap());
        if bit_time.num_microseconds().unwrap() > 45 {
            data.push(1);
        } else {
            data.push(0);
        };
    }
    let hum_bit = Vec::from(&data[0..8]);
    let hum_dec_bit = Vec::from(&data[8..16]);
    let temp_bit = Vec::from(&data[16..24]);
    let temp_dec_bit = Vec::from(&data[24..32]);
    let check_bit = Vec::from(&data[32..40]);
    let mut hum = 0;
    let mut hum_dec = 0;
    let mut temp = 0;
    let mut temp_dec = 0;
    let mut check = 0;

    for i in 0..8 {
        hum += hum_bit[i] * i32::pow(2, 7 - i as u32);
        hum_dec += hum_dec_bit[i] * i32::pow(2, 7 - i as u32);
        temp += temp_bit[i] * i32::pow(2, 7 - i as u32);
        temp_dec += temp_dec_bit[i] * i32::pow(2, 7 - i as u32);
        check += check_bit[i] * i32::pow(2, 7 - i as u32);
    }
    if check != hum + hum_dec + temp + temp_dec {
        println!("Error reading temp/humidity");
        // println!("check {}\ntest {}\n", check, hum + hum_dec + temp + temp_dec);
    };
    println!("temp {}.{}\nhumidity {}.{}\n", temp, temp_dec, hum, hum_dec);
    let hum = f32::from_str(&format!("{}.{}", hum, hum_dec)).expect("should be float");
    let temp = f32::from_str(&format!("{}.{}", temp, temp_dec)).expect("should be float");
    (hum, temp)
}
