use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::cmp::{max, min};
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

    /// Initiate message with 500 us pulse.
    ///
    /// Transmit message; long pulse = 1 short pulse = 0.
    ///
    /// Terminate message with 1000 us pulse.
    pub fn send_message(&mut self, message: String) {
        let encoded_message = self.encode_message(message);

        // Initiation sequence.
        self.out.set_value(false).expect("Error setting pin");
        thread::sleep(Duration::from_micros(50));
        self.out.set_value(true).expect("Error setting pin");
        thread::sleep(Duration::from_micros(500));
        self.out.set_value(false).expect("Error setting pin");
        thread::sleep(Duration::from_micros(50));

        // Begin message transmission.
        for bit in encoded_message {
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

    /// Loop while checking for initiation sequence.
    /// Push 1 for long pulse, 0 for short.
    /// Return data upon termination sequence
    fn receive_message(&mut self) -> Vec<u32> {
        let mut data = Vec::new();
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
            if (400 < initiation_time) && (initiation_time < 600) {
                break;
            }
        }
        println!("\nIncoming message detected...\n");
        // Data reception
        'outer: loop {
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
                i64::MIN..=-0_i64 => continue,
                1..=95 => data.push(0),
                96..=200 => data.push(1),
                201..=999 => continue,
                1000.. => break 'outer, // Termination sequence.
            };
        }
        data
    }

    /// Decode binary into string and return.
    /// Add char codes and return boolean comparison with check_sum.
    /// Return error.
    fn decode(&mut self, data: &Vec<u32>) -> (String, bool, f32) {
        let data_len = data.len();
        // Min one byte message plus check sum.
        if data.len() < 40 {
            return ("".to_string(), false, 0.0);
        }
        let mut sum: u32 = 0;
        let mut message = "".to_string();

        // Get int from each byte, convert to char, append to message.
        for i in (0..data_len - 32).step_by(8) {
            let mut byte = 0;
            for j in 0..8 {
                byte += data[i + j] << j as u32;
            }
            sum += byte;
            message = message + &format!("{}", char::from_u32(byte).expect("Error decoding char"));
        }

        // Get check sum.
        let mut check: u32 = 0;
        for (i, bit) in data[data_len - 32..data_len].iter().enumerate() {
            check += *bit << i;
        }
        // VERY roughly estimate data fidelity.
        let min = min(sum, check) as f32;
        let max = max(sum, check) as f32;
        let error = min / max;
        (message, error > 0.99, error)
    }

    /// Call receive and decode methods.
    /// Char codes -> chars -> String.
    /// Print to stdout
    /// Return num kilobytes, seconds and error
    pub fn print_message(&mut self) -> (f32, f64, f32) {
        let start = chrono::Utc::now();
        println!("\nAwaiting transmission...");
        let data = self.receive_message();

        println!("Message received. Validating...\n");
        let (message, valid, error) = self.decode(&data);

        let num_kbytes = message.clone().len() as f32 / 1000.0;
        let end = chrono::Utc::now();
        let seconds = (end - start).num_milliseconds() as f64 / 1000.0f64;
        if !valid {
            println!("ERROR: Invalid data detected.\n\n");
            return (num_kbytes, seconds, error);
        }
        println!("Validated message:\n\n{}\n\n", message);
        (num_kbytes, seconds, error)
    }
}
