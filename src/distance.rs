use gpio::{GpioIn, GpioOut};
use gpio::GpioValue::{High, Low};
use std::time::Duration;
use std::{thread};


pub struct Distance {
    in_: gpio::sysfs::SysFsGpioInput,
    out: gpio::sysfs::SysFsGpioOutput,
}

impl Distance {
    pub fn new() -> Distance {
        let mut in_ = gpio::sysfs::SysFsGpioInput::open(24).unwrap();
        let mut out = gpio::sysfs::SysFsGpioOutput::open(23).unwrap();
        out.set_value(false).unwrap();
        thread::sleep(Duration::from_secs(2));
        Self { in_, out }
    }

    pub fn measure(&mut self) -> f64 {
        self.out.set_value(true).unwrap();
        thread::sleep(Duration::from_micros(15));
        self.out.set_value(false).unwrap();
        while self.in_.read_value().unwrap() == Low {
            continue
        }
        let t1 = chrono::Utc::now();
        while self.in_.read_value().unwrap() == High {
            continue
        }
        let t2 = chrono::Utc::now();
        let distance = (t2 - t1).num_microseconds().unwrap() as f64 * 340.0 / 2.0;
        distance /10_000
    }
}