use gpio::GpioValue::{High, Low};
use gpio::GpioOut;
use std::thread;
use std::time::Duration;

const RCLK:u16 = 17;
const SRCLK:u16 = 27;
const SDI:u16 = 22;

pub struct DotMatrix {
    rclk: gpio::sysfs::SysFsGpioOutput,
    srclk: gpio::sysfs::SysFsGpioOutput,
    sdi: gpio::sysfs::SysFsGpioOutput,
}

impl DotMatrix {
    pub fn new() -> DotMatrix {
        let mut rclk = gpio::sysfs::SysFsGpioOutput::open(RCLK).expect("Pin should be active");
        rclk.set_value(false).expect("Pin should set.");
        let mut srclk = gpio::sysfs::SysFsGpioOutput::open(SRCLK).expect("Pin should be active");
        srclk.set_value(false).expect("PIn should set.");
        let mut sdi = gpio::sysfs::SysFsGpioOutput::open(SDI).expect("Pin should be active");
        sdi.set_value(false).expect("Pin should set.");

        Self {
            rclk,
            srclk,
            sdi,
        }
    }

    fn input(&mut self, data: u8) {
        for i in 0..8 {
            match 0x80 & (data << i) {
                1 => self.sdi.set_value(High).expect("bit should set"),
                0 => self.sdi.set_value(Low).expect("bit should set"),
                _ => println!("bit {}", 0x80 & (data << i))
            };
            self.srclk.set_value(High).expect("bit should set");
            self.srclk.set_value(Low).expect("bit should set");
        }
    }

    fn output(&mut self) {
        self.rclk.set_value(High).expect("pin should set");
        self.rclk.set_value(Low).expect("pin should set");
    }

    fn display_data(&mut self, data: [u8; 8], tab: [u8; 8]) {
        for i in 0..data.len() {
            for _ in 0..15 {
                for j in 0..8 {
                    self.input(data[i + j]);
                    self.input(tab[j]);
                    self.output();
                    thread::sleep(Duration::from_millis(2));
                }
            }
        }
    }

    pub fn test(&mut self) {
        let data = [0x00,0x00,0x3C,0x42,0x42,0x3C,0x00,0x00];
        let tab = [0xfe,0xfd,0xfb,0xf7,0xef,0xdf,0xbf,0x7f];
        self.display_data(data, tab);
    }
}