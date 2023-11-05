use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::thread;
use std::time::Duration;

const LASER_PIN: u16 = 18;
const RECIEVER_PIN: u16 = 23;

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
        self.out.set_value(false).unwrap();
        thread::sleep(Duration::from_millis(200));
        //  self.out.set_value(true).unwrap();
        // thread::sleep(Duration::from_millis(20));
        // self.out.set_value(false).unwrap();
        // thread::sleep(Duration::from_millis(10));
        let mut data =  Vec::new();
        for char in message.chars() {
            let code = char as i8;
            // println!("char {}\ncode {}\n", char, code);
            for bit in (0..8).map(|n| (code >> n) & 1) {
                data.push(bit);
                match bit == 1 {
                    true => {
                        self.out.set_value(true).unwrap();
                        thread::sleep(Duration::from_millis(5));
                        self.out.set_value(false).unwrap();
                    }
                    false => {
                        self.out.set_value(true).unwrap();
                        thread::sleep(Duration::from_millis(2));
                        self.out.set_value(false).unwrap();
                    }
                }
            }
            thread::sleep(Duration::from_millis(5))
        }
        // println!("output \n{:?}", data);
    }
}

impl Receiver {
    pub fn new() -> Receiver {
        let in_ = gpio::sysfs::SysFsGpioInput::open(RECIEVER_PIN).unwrap();
        Self { in_ }
    }

    fn receive_message(&mut self) -> Vec<u32> {
        let mut data = Vec::new();
        // while self.in_.read_value().unwrap() == High {
        //     continue;
        // }
        // while self.in_.read_value().unwrap() == Low {
        //     continue;
        // }
        for _ in 0..96 {
            while self.in_.read_value().unwrap() == Low {
                continue;
            }
            let start = chrono::Utc::now();
            while self.in_.read_value().unwrap() == High {
                continue;
            }
            let end = chrono::Utc::now();
            let bit_time = (end - start).num_milliseconds();
            println!("bit time {}", bit_time);
            if bit_time > 3 {
                data.push(1);
            } else {
                data.push(0);
            };
        }
        data
    }

    pub fn print_message(&mut self) {
        println!("getting data");
        let data = self.receive_message();
        println!("decoding data: \n{:?}", data);
        let mut chars = Vec::new();
        for i in (0..data.len() - 1).step_by(8) {
            let mut code: u32 = 0;
            for j in 0..8 {
                code += data[i + j] * u32::pow(2, 7 - j as u32);
            }
            chars.push(char::from_u32(code))
        }
        let mut message: String = "".to_string();
        for char in chars {
            match char {
                Some(char) => message = message + &format!("{}", char),
                None => continue,
            }
        }
        println!("message: {}\nlen {}", message, message.len())
    }
}
