use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::thread;
use std::time::Duration;

pub struct Distance {
    in_: gpio::sysfs::SysFsGpioInput,
    out: gpio::sysfs::SysFsGpioOutput,
}

impl Distance {
    pub fn new() -> Distance {
        let in_ = gpio::sysfs::SysFsGpioInput::open(24).unwrap();
        let mut out = gpio::sysfs::SysFsGpioOutput::open(23).unwrap();
        out.set_value(false).unwrap();
        thread::sleep(Duration::from_secs(2));
        Self { in_, out }
    }

    fn measure(&mut self) -> f64 {
        self.out.set_value(true).unwrap();
        thread::sleep(Duration::from_micros(15));
        self.out.set_value(false).unwrap();
        while self.in_.read_value().unwrap() == Low {
            continue;
        }
        let t1 = chrono::Utc::now();
        while self.in_.read_value().unwrap() == High {
            continue;
        }
        let t2 = chrono::Utc::now();
        let distance = (t2 - t1).num_microseconds().unwrap() as f64 * 340.0 / 2.0;
        distance
    }

    pub fn print_measure(&mut self) -> String {
        let reading = self.measure();
        // println!("reading {}", reading);
        let mut print_string = reading.to_string();
        let num_pad = 8 - print_string.len();
        for _ in 0..num_pad {
            print_string = "0".to_string() + &print_string;
        }
        print_string[1..5].to_string()
    }
}
