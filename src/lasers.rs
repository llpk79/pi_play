use crate::huffman_code::HuffTree;
use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::cmp::{max, min};
use std::thread;
use std::time::Duration;

const LASER_PIN: u16 = 18;
const RECEIVER_PIN: u16 = 23;

pub struct Laser {
    out: gpio::sysfs::SysFsGpioOutput,
    huff_tree: HuffTree,
}

pub struct Receiver {
    in_: gpio::sysfs::SysFsGpioInput,
    huff_tree: HuffTree,
}

impl Laser {
    pub fn new(huff_tree: HuffTree) -> Laser {
        // Open port for laser pin.
        let out = match gpio::sysfs::SysFsGpioOutput::open(LASER_PIN) {
            Ok(out) => out,
            Err(_e) => panic!(),
        };
        Self {
            out,
            huff_tree,
        }
    }

    /// Initiate message with 500 microsecond pulse.
    ///
    /// Transmit message; long pulse = 1 short pulse = 0.
    ///
    /// Terminate message with 1000 microsecond pulse.
    pub fn send_message(&mut self, message: String) {
        let encoded_message = self.huff_tree.encode(&message);
        // Initiation sequence.
        self.out.set_value(false).expect("Pin should be active");
        thread::sleep(Duration::from_millis(5));
        self.out.set_value(true).expect("Pin should be active");
        thread::sleep(Duration::from_millis(10));
        self.out.set_value(false).expect("Pin should be active");
        thread::sleep(Duration::from_millis(5));

        // Begin message transmission.
        for bit in encoded_message {
            match bit == 1 {
                true => {
                    self.out.set_value(true).expect("Pin should be active");
                    thread::sleep(Duration::from_millis(3));
                    self.out.set_value(false).expect("Pin should be active");
                }
                false => {
                    self.out.set_value(true).expect("Pin should be active");
                    thread::sleep(Duration::from_millis(1));
                    self.out.set_value(false).expect("Pin should be active");
                }
            }
            // Bit resolution. Gets sloppy below 50 microseconds.
            thread::sleep(Duration::from_millis(3))
        }

        // Termination sequence.
        self.out.set_value(true).expect("Pin should be active");
        thread::sleep(Duration::from_millis(15));
        self.out.set_value(false).expect("Pin should be active");
    }
}

impl Receiver {
    pub fn new(huff_tree: HuffTree) -> Receiver {
        // Open port for receiver pin.
        let in_ = match gpio::sysfs::SysFsGpioInput::open(RECEIVER_PIN) {
            Ok(in_) => in_,
            Err(_e) => panic!(),
        };
        Self { in_, huff_tree }
    }

    /// Loop until initiation sequence is detected.
    fn detect_message(&mut self) {
        loop {
            while self.in_.read_value().expect("Pin should be active") == Low {
                continue;
            }
            // Get the amount of time the laser is on.
            let begin = chrono::Utc::now();
            while self.in_.read_value().expect("Pin should be active") == High {
                continue;
            }
            let initiation_time = (chrono::Utc::now() - begin)
                .num_milliseconds();
                // .expect("Some time should have passed");
            match initiation_time {
                i64::MIN..=8 => continue,
                9..=11 => break,
                12.. => continue,
            }
        }
    }

    /// Push 1 for long pulse, 0 for short.
    ///
    /// Return data upon termination sequence.
    fn receive(&mut self) -> Vec<u32> {
        let mut data = Vec::new();
        loop {
            while self.in_.read_value().expect("Pin should be active") == Low {
                continue;
            }
            // Get the amount of time the laser is on.
            let start = chrono::Utc::now();
            while self.in_.read_value().expect("Pin should be active") == High {
                continue;
            }
            let bit_time = (chrono::Utc::now() - start)
                .num_milliseconds();
                // .expect("Some time should have passed");
            // println!("bit time {}", bit_time);
            match bit_time {
                i64::MIN..=-0 => continue,
                1..=2 => data.push(0),
                3..=4 => data.push(1),
                5..=10 => continue, // Bad data, we could guess, I guess?
                11.. => break,       // Termination sequence.
            };
        }
        data
    }

    /// Last 32 bits contain checksum.
    ///
    /// Sum each 8 bit word in message and compare to checksum.
    ///
    /// Return comparison and error.
    fn validate(&self, data: &Vec<u32>) -> (bool, f32) {
        let data_len = data.len();
        // Min one byte message plus checksum.
        if data_len < 40 {
            return (false, 0.0);
        }
        let mut sum: u32 = 0;

        // Get int from each byte.
        for i in (0..data_len - 32).step_by(8) {
            let mut byte = 0;
            for bit in (0..8).map(|j| data[i + j] << j) {
                byte += bit
            }
            sum += byte;
        }

        // Get checksum.
        let mut check: u32 = 0;
        for (i, bit) in data[data_len - 32..].iter().enumerate() {
            check += *bit << i;
        }
        // VERY roughly estimate data fidelity.
        let min = min(sum, check) as f32;
        let max = max(sum, check) as f32;
        let error = min / max;
        (error > 0.95, error)
    }

    /// Call detect, receive and decode methods.
    ///
    /// Print to stdout.
    pub fn receive_message(&mut self) -> Vec<String> {
        println!("\n\nAwaiting transmission...");
        self.detect_message();
        let start = chrono::Utc::now();

        println!("\nIncoming message detected...\n");
        let data = self.receive();

        println!("Message received. Validating...\n");
        let (valid, error) = self.validate(&data);

        let end = chrono::Utc::now();
        return match valid {
            true => {
                let sans_checksum = Vec::from(&data[0..(data.len() - 32)]);
                let message = self.huff_tree.decode(sans_checksum);
                let pre_lcd_message: Vec<&str> = message.split("\n").collect();
                let lcd_message: Vec<String> = pre_lcd_message.iter().map(|s| s.to_string()).collect();

                // Calculate stats
                let seconds = (end - start).num_milliseconds() as f64 / 1000.0_f64;
                let decode_time =
                    (chrono::Utc::now() - end).num_microseconds().unwrap() as f64 / 1000_000.0_f64;
                println!(
                    "Message in {:.3} sec\nDecode in {:.6} sec\nKB {:.3}\n'Error' {:.6}\n",
                    seconds,
                    decode_time,
                    lcd_message.len() as f32 / 1000_f32,
                    1.0 - error,
                );

                lcd_message
            }
            false => {
                println!("ERROR: Invalid data detected.\n");

                Vec::from(["".to_string()])
            }
        };

    }
}
