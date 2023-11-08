use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::cmp::{max, min};
use std::time::Duration;
use std::{fs, thread};

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
        let out = match gpio::sysfs::SysFsGpioOutput::open(LASER_PIN) {
            Ok(out) => out,
            Err(_e) => panic!(),
        };
        Self { out }
    }

    fn encode_message(&mut self, message: String) -> Vec<u8> {
        let mut data = Vec::new();
        let mut check_sum: i32 = 0;
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
        thread::sleep(Duration::from_micros(1000));
        self.out.set_value(true).expect("Error setting pin");
        thread::sleep(Duration::from_micros(1000));
        self.out.set_value(false).expect("Error setting pin");
        thread::sleep(Duration::from_micros(500));
        let encoded_message = self.encode_message(message);
        // Begin message transmission.
        for bit in encoded_message {
            match bit == 1 {
                true => {
                    self.out.set_value(true).expect("Error setting pin");
                    thread::sleep(Duration::from_micros(50));
                    self.out.set_value(false).expect("Error setting pin");
                }
                false => {
                    self.out.set_value(true).expect("Error setting pin");
                    thread::sleep(Duration::from_micros(25));
                    self.out.set_value(false).expect("Error setting pin");
                }
            }
            thread::sleep(Duration::from_micros(50))
        }

        // Termination sequence.
        self.out.set_value(true).expect("Error setting pin");
        thread::sleep(Duration::from_micros(1500));
        self.out.set_value(false).expect("Error setting pin");
        thread::sleep(Duration::from_micros(100));
    }
}

impl Receiver {
    pub fn new() -> Receiver {
        let in_ = match gpio::sysfs::SysFsGpioInput::open(RECEIVER_PIN) {
            Ok(in_) => in_,
            Err(_e) => panic!(),
        };
        Self { in_ }
    }

    fn receive_message(&mut self) -> Vec<u32> {
        let mut data = Vec::new();
        println!("\nAwaiting transmission...");
        loop {
            // Detect initiation sequence.
            while self.in_.read_value().expect("Error reading pin") == Low {
                continue;
            }
            let begin = chrono::Utc::now();
            while self.in_.read_value().expect("Error reading pin") == High {
                continue;
            }
            let end = chrono::Utc::now();
            let initiation_time = (end - begin).num_microseconds().expect("micro");
            if (900 < initiation_time) && (initiation_time < 1400) {
                break;
            }
        }
        println!("\nIncoming message detected...\n");
        // Data reception
        'outer: loop {
            while self.in_.read_value().expect("Error reading pin") == Low {
                continue;
            }
            let start = chrono::Utc::now();
            while self.in_.read_value().expect("Error reading pin") == High {
                continue;
            }
            let end = chrono::Utc::now();
            let bit_time = (end - start).num_microseconds().expect("micro");
            // println!("bit time {}", bit_time);
            match bit_time {
                i64::MIN..=-0_i64 => continue,
                1..=100 => data.push(0),
                101..=900 => data.push(1),
                901..=1500 => {
                    continue;
                }
                1501.. => break 'outer, // Termination sequence.
            };
        }

        data
    }

    fn validate(&mut self, data: &Vec<u32>) -> (Vec<u32>, bool) {
        let data_len = data.len();
        if data.len() < 40 {
            return (Vec::from([0]), false);
        }
        let mut check: u32 = 0;
        let mut sum: u32 = 0;
        let mut codes: Vec<u32> = Vec::new();
        for i in (0..data_len - 32).step_by(8) {
            let mut byte = 0;
            for j in 0..8 {
                if i + j >= data_len {
                    // I do not know why this happens sometimes.
                    break;
                }
                byte += data[i + j] << j as u32;
            }
            sum += byte;
            codes.push(byte);
        }
        for (i, code) in data[data_len - 32..data_len].iter().enumerate() {
            check += *code << i;
        }
        // VERY roughly estimate data fidelity.
        let min = min(sum, check) as f32;
        let max = max(sum, check) as f32;
        (codes, min / max > 0.95)
    }

    pub fn print_message(&mut self) -> (f32, i64) {
        let start = chrono::Utc::now();
        let data = self.receive_message();
        println!("Message received. Validating...\n");
        let (codes, valid) = self.validate(&data);
        let num_kbytes = codes.clone().len() as f32 / 1000.0;
        if !valid {
            println!("ERROR: Invalid data detected.\n\n");
            return (num_kbytes, (chrono::Utc::now() - start).num_seconds());
        }
        let mut message: String = "".to_string();
        for code in codes {
            match char::from_u32(code) {
                Some(char) => message = message + &format!("{}", char),
                None => continue,
            }
        }
        fs::write("./test.txt", &message).expect("file not written");
        let end = chrono::Utc::now();
        let seconds = (end - start).num_seconds();
        println!("Validated message:\n\n{}\n\n", message);
        (num_kbytes, seconds)
    }
}
