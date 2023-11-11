use std::cmp::{max, min};
use crate::huffman_code::HuffTree;
use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
// use std::cmp::{max, min};
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
    /// Open port for laser pin.
    pub fn new() -> Laser {
        let out = match gpio::sysfs::SysFsGpioOutput::open(LASER_PIN) {
            Ok(out) => out,
            Err(_e) => panic!(),
        };
        Self { out }
    }

    /// String -> char code -> `[bits]`.
    /// Sum char codes to 32 bit int and append to data as check_sum.
    fn add_checksum(&mut self, data: &mut Vec<u32>) -> Vec<u32> {
        // Add char code to checksum, push char data bitwise.
        let mut check_sum = 0;
        for _ in 0..(data.len() + 1 % 8) {
            data.push(0);
        }
        for i in (0..data.len() - 1).step_by(8) {
            let mut byte = 0;
            for j in 0..8 {
                byte += data[i + j] << j;
            }
            check_sum += byte as i32;
        }
        // Push checksum data bitwise.
        let mut check_vec = Vec::new();
        for bit in (0..32).map(|n| (check_sum >> n) & 1) {
            check_vec.push(bit as u32);
        }
        Vec::from([data.clone(), check_vec].concat())
    }

    /// Initiate message with 500 us pulse.
    ///
    /// Transmit message; long pulse = 1 short pulse = 0.
    ///
    /// Terminate message with 1000 us pulse.
    pub fn send_message(&mut self, message: &mut Vec<u32>) {
        let message = self.add_checksum(message);

        // Initiation sequence.
        self.out.set_value(false).expect("Error setting pin");
        thread::sleep(Duration::from_micros(50));
        self.out.set_value(true).expect("Error setting pin");
        thread::sleep(Duration::from_micros(500));
        self.out.set_value(false).expect("Error setting pin");
        thread::sleep(Duration::from_micros(50));

        // Begin message transmission.
        for bit in message {
            match bit == 1 {
                true => {
                    self.out.set_value(true).expect("Error setting pin");
                    thread::sleep(Duration::from_micros(25));
                    self.out.set_value(false).expect("Error setting pin");
                }
                false => {
                    self.out.set_value(true).expect("Error setting pin");
                    thread::sleep(Duration::from_micros(10));
                    self.out.set_value(false).expect("Error setting pin");
                }
            }
            // Bit resolution.
            thread::sleep(Duration::from_micros(50))
        }

        // Termination sequence.
        self.out.set_value(true).expect("Error setting pin");
        thread::sleep(Duration::from_micros(1000));
        self.out.set_value(false).expect("Error setting pin");
    }
}

impl Receiver {
    /// Open port for receiver pin.
    pub fn new() -> Receiver {
        let in_ = match gpio::sysfs::SysFsGpioInput::open(RECEIVER_PIN) {
            Ok(in_) => in_,
            Err(_e) => panic!(),
        };
        Self { in_ }
    }

    /// Loop until initiation sequence is detected.
    fn detect_message(&mut self) {
        // Detect initiation sequence.
        loop {
            while self.in_.read_value().expect("Error reading pin") == Low {
                continue;
            }
            // Get the amount of time the laser is on.
            let begin = chrono::Utc::now();
            while self.in_.read_value().expect("Error reading pin") == High {
                continue;
            }
            let end = chrono::Utc::now();
            let initiation_time = (end - begin).num_microseconds().expect("micro");
            match initiation_time {
                i64::MIN..=400 => continue,
                401..=900 => break,
                901.. => continue,
            }
        }
    }

    /// Push 1 for long pulse, 0 for short.
    /// Return data upon termination sequence
    fn receive_message(&mut self) -> Vec<u32> {
        let mut data = Vec::new();
        // Data reception
        loop {
            while self.in_.read_value().expect("Error reading pin") == Low {
                continue;
            }
            // Get the amount of time the laser is on.
            let start = chrono::Utc::now();
            while self.in_.read_value().expect("Error reading pin") == High {
                continue;
            }
            let end = chrono::Utc::now();
            let bit_time = (end - start).num_microseconds().expect("micro");
            // println!("bit time {}", bit_time);
            match bit_time {
                i64::MIN..=-0 => continue,
                1..=95 => data.push(0),
                96..=200 => data.push(1),
                201..=999 => continue,
                1000.. => break, // Termination sequence.
            };
        }
        data
    }

    /// Sum char codes and return boolean comparison with checksum.
    /// Return error.
    fn validate(&mut self, data: &Vec<u32>) -> (bool, f32) {
        let data_len = data.len();
        // Min one byte message plus checksum.
        if data_len < 40 {
            return (false, 0.0);
        }
        let mut sum: u32 = 0;

        // Get int from each byte, convert to char, append to message.
        for i in (0..data_len - 32).step_by(8) {
            let mut byte = 0;
            for j in 0..8 {
                byte += data[i + j] << j as u32;
            }
            sum += byte;
        }

        // Get checksum.
        let mut check: u32 = 0;
        for (i, bit) in data[data_len - 32..data_len].iter().enumerate() {
            check += *bit << i;
        }
        // VERY roughly estimate data fidelity.
        let min = min(sum, check) as f32;
        let max = max(sum, check) as f32;
        let error = min / max;
        (error > 0.995, error)
    }

    /// Call receive and decode methods.
    /// Print to stdout
    pub fn print_message(&mut self, huff_tree: &mut HuffTree) {
        println!("\nAwaiting transmission...");
        self.detect_message();
        let start = chrono::Utc::now();

        println!("\nIncoming message detected...\n");
        let data = self.receive_message();

        let (valid, error) = self.validate(&data);
        let message = huff_tree.decode(data);
        println!("Message received. Validating...\n");
        match valid {
            true => println!("Validated message:\n\n{}\n\n", message),
            false => println!("ERROR: Invalid data detected.\n\n"),
        }

        let num_kbytes = message.clone().len() as f32 / 1000.0;
        let end = chrono::Utc::now();
        let seconds = (end - start).num_milliseconds() as f64 / 1000.0f64;

        println!(
            "Message in {:.3} sec\nKB/s {:.3}\n'Error' {:.5}",
            seconds,
            num_kbytes as f64 / seconds,
            1.0 - error,
        );
    }
}
