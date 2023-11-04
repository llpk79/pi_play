use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::{thread};
use std::time::Duration;
use std::time;

const LASER_PIN:u16 = 18;
const RECIEVER_PIN:u16 = 23;

pub struct Laser {
    out: gpio::sysfs::SysFsGpioOutput,
}

pub struct Receiver {
    in_: gpio::sysfs::SysFsGpioInput,
}

impl Laser {
    pub fn new() -> Laser {
        let out = gpio::sysfs::SysFsGpioOutput::open(LASER_PIN).unwrap();
        Self { out }
    }

    pub fn send_message(&mut self, message: String) {
        thread::sleep(Duration::from_millis(250));
        self.out.set_value(false).unwrap();
        thread::sleep(Duration::from_millis(20));
        self.out.set_value(true).unwrap();
        thread::sleep(Duration::from_millis(20));
        self.out.set_value(false).unwrap();
        loop {
            for char in message.chars() {
                println!("sent {}", char);
                let code = char as i8;
                for bit in code.to_le_bytes() {
                    match bit == 1 {
                        true => {
                            self.out.set_value(true).unwrap();
                            thread::sleep(Duration::from_millis(2500))
                        }
                        false => {
                            self.out.set_value(true).unwrap();
                            thread::sleep(Duration::from_micros(5000))
                        }
                    }
                }
            }
        }
    }
}

impl Receiver {
    pub fn new() -> Receiver {
        let in_ = gpio::sysfs::SysFsGpioInput::open(RECIEVER_PIN).unwrap();
        Self { in_ }
    }

    fn receive_message(&mut self) -> Vec<u32>{
        let mut data = Vec::new();
        while self.in_.read_value().unwrap() == Low {
            continue;
        };
        while self.in_.read_value().unwrap() == High {
            continue;
        };
        while data.len() < 40 {
            while self.in_.read_value().unwrap() == Low {
                continue;
            };
            let start = chrono::Utc::now();
            while self.in_.read_value().unwrap() == High {
                continue;
            };
            let end = chrono::Utc::now();
            let bit_time = end - start;
            println!("bit time {:?}", bit_time.num_microseconds().unwrap());
            if bit_time.num_milliseconds() > 5000 {
                data.push(1);
            } else {
                data.push(0);
            };
        }data
    }

    pub fn print_message(&mut self) {
        let data = self.receive_message();
        let mut chars = Vec::new();
        for i in (0..data.len()).step_by(8) {
            let mut code:u32 = 0;
            for j in 0..8 {
                code += data[i + j] * u32::pow(2, 7 - j as u32);
            }
            chars.push(char::from_u32(code))
        }
        let mut message:String = "".to_string();
        for char in chars {
            match char {
                Some(char) => message = message + &format!("{}", char),
                None => continue
            }
        }
        println!("{}", message)
    }
}