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

    fn encode_message(&mut self, message: String) -> Vec<u8> {
        let mut data = Vec::new();
        let mut check_sum:i32 = 0;
        for char in message.chars() {
            let code = char as u8;
            check_sum += code as i32;
            for bit in (0..8).map(|n| (code >> n) & 1) {
                data.push(bit);
            }
        }
        for bit in (0..32).map(|n| (check_sum >> n) & 1) {
            data.push(bit as u8);
        }
        data
    }

    pub fn send_message(&mut self, message: String) {
        // Initiation sequence.
        thread::sleep(Duration::from_micros(500));
        self.out.set_value(true).unwrap();
        thread::sleep(Duration::from_micros(500));
        self.out.set_value(false).unwrap();
        thread::sleep(Duration::from_micros(400));
        let encoded_message = self.encode_message(message);
        // Begin message transmission.
        for bit in encoded_message {
            match bit == 1 {
                true => {
                    self.out.set_value(true).unwrap();
                    thread::sleep(Duration::from_micros(200));
                    self.out.set_value(false).unwrap();
                }
                false => {
                    self.out.set_value(true).unwrap();
                    thread::sleep(Duration::from_micros(100));
                    self.out.set_value(false).unwrap();
                }
            }
            thread::sleep(Duration::from_micros(100))
        }

        // Termination sequence.
        self.out.set_value(true).unwrap();
        thread::sleep(Duration::from_micros(750));
        self.out.set_value(false).unwrap();
        thread::sleep(Duration::from_micros(100));
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
        if (450 < initiation_time) && (initiation_time < 550) {
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
                    1..=200 => data.push(0),
                    201..=320 => data.push(1),
                    321..=750 => continue,
                    751.. => break 'outer, // Termination sequence.
                };
            }
        }
        data
    }

    fn validate(&mut self, data: &Vec<u32>) -> bool {
        let mut check:u32 = 0;
        let mut sum:u32 = 0;
        for i in (0..data.len() - 32).step_by(8) {
            let mut byte = 0;
            for j in 0..8 {
                if i + j >= data.len() {
                    // I do not know why this happens sometimes.
                    break;
                }
                byte += data[i + j] << j as u32;
            }
            sum += byte;
        }
        for (i, code) in data[data.len() - 32..data.len()].iter().enumerate() {
            check += *code << i;
        }
        sum == check
    }

    pub fn print_message(&mut self) {
        let data = self.receive_message();
        println!("Message received. validating...\n");
        if data.len() < 40 {
            return;
        }
        if !self.validate(&data) {
            println!("ERROR: Invalid data detected.\n\n");
            return;
        }
        let mut chars = Vec::new();
        let mut codes = Vec::new();
        for i in (0..data.len() - 32).step_by(8) {
            let mut code: u32 = 0;
            for j in 0..8 {
                code += data[i + j] << j;
            }
            codes.push(code as u8);
            chars.push(char::from_u32(code))
        }
        let mut message: String = "".to_string();
        for char in chars {
            match char {
                Some(char) => message = message + &format!("{}", char),
                None => continue,
            }
        }
        println!("Validated message:\n\t{}\n\n", message);
    }
}
