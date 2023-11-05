use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::thread;
use std::time::Duration;

const LASER_PIN: u16 = 18;
const RECEIVER_PIN: u16 = 23;

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
        // Initiation sequence.
        thread::sleep(Duration::from_micros(200));
        self.out.set_value(true).unwrap();
        thread::sleep(Duration::from_micros(200));
        self.out.set_value(false).unwrap();
        thread::sleep(Duration::from_micros(100));
        let mut data = Vec::new();

        // Begin message transmission.
        for char in message.chars() {
            let code = char as i8;
            for bit in (0..8).map(|n| (code >> n) & 1) {
                data.push(bit);
                match bit == 1 {
                    true => {
                        self.out.set_value(true).unwrap();
                        thread::sleep(Duration::from_micros(60));
                        self.out.set_value(false).unwrap();
                    }
                    false => {
                        self.out.set_value(true).unwrap();
                        thread::sleep(Duration::from_micros(20));
                        self.out.set_value(false).unwrap();
                    }
                }
            }
            thread::sleep(Duration::from_micros(200))
        }

        // Termination sequence.
        self.out.set_value(true).unwrap();
        thread::sleep(Duration::from_micros(300));
        self.out.set_value(false).unwrap();
        thread::sleep(Duration::from_micros(20));
    }
}

impl Receiver {
    pub fn new() -> Receiver {
        let in_ = gpio::sysfs::SysFsGpioInput::open(RECEIVER_PIN).unwrap();
        Self { in_ }
    }

    fn receive_message(&mut self) -> Vec<u32> {
        let mut data = Vec::new();

        // Detect initiation sequence.
        while self.in_.read_value().unwrap() == Low {
            continue;
        }
        let begin = chrono::Utc::now();
        while self.in_.read_value().unwrap() == High {
            continue;
        }
        let end = chrono::Utc::now();
        let initiation_time = (end - begin).num_microseconds().unwrap();
        if (180 < initiation_time) && (initiation_time < 220) {
            println!("Incoming message detected...\n");
            // Data reception
            'outer: loop {
                while self.in_.read_value().unwrap() == Low {
                    continue;
                }
                let start = chrono::Utc::now();
                while self.in_.read_value().unwrap() == High {
                    continue;
                }
                let end = chrono::Utc::now();
                let bit_time = (end - start).num_microseconds().unwrap();
                println!("bit time {}", bit_time);
                match bit_time {
                    i64::MIN..=-0_i64 => continue,
                    1..=100 => data.push(0),
                    101..=200 => data.push(1),
                    201..=330 => continue,
                    331.. => break 'outer,  // Termination sequence.
                };
            }
        }
        data
    }

    pub fn print_message(&mut self) {
        let data = self.receive_message();
        if data.len() < 8 {
            return;
        }
        println!("Message received. Decoding...");
        let mut chars = Vec::new();
        let mut codes = Vec::new();
        for i in (0..data.len() - 1).step_by(8) {
            let mut code: u32 = 0;
            for j in 0..8 {
                if i + j >= data.len() {  // I do not know why this happens sometimes.
                    break;
                }
                code += data[i + j] * u32::pow(2, j as u32);
            }
            codes.push(code);
            chars.push(char::from_u32(code))
        }
        let mut message: String = "".to_string();
        for char in chars {
            match char {
                Some(char) => message = message + &format!("{}", char),
                None => continue,
            }
        }
        println!("message: {}\n", message)
    }
}
