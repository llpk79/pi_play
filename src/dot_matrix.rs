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
        rclk.set_value(Low).expect("Pin should set.");
        let mut srclk = gpio::sysfs::SysFsGpioOutput::open(SRCLK).expect("Pin should be active");
        srclk.set_value(Low).expect("PIn should set.");
        let mut sdi = gpio::sysfs::SysFsGpioOutput::open(SDI).expect("Pin should be active");
        sdi.set_value(Low).expect("Pin should set.");

        Self {
            rclk,
            srclk,
            sdi,
        }
    }

    fn input(&mut self, data: u8) {
        for i in 0..8 {
            match 0x80 & (data << i) {
                0 => self.sdi.set_value(Low).expect("bit should set"),
                _ => self.sdi.set_value(High).expect("bit should set"),
            };
            self.srclk.set_value(High).expect("bit should set");
            self.srclk.set_value(Low).expect("bit should set");
        }
    }

    fn output(&mut self) {
        self.rclk.set_value(High).expect("pin should set");
        self.rclk.set_value(Low).expect("pin should set");
    }

    fn display_data(&mut self, data: [u8; 24], tab: [u8; 8]) {
        for i in 0..16 {
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
        let data = [
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, //NULL
            // 0x00,0x00,0x3C,0x42,0x42,0x3C,0x00,0x00, //#0
            // 0x00,0x00,0x00,0x44,0x7E,0x40,0x00,0x00, //1
            // 0x00,0x00,0x44,0x62,0x52,0x4C,0x00,0x00, //2
            // 0x00,0x00,0x78,0x14,0x12,0x14,0x78,0x00, //A
            // 0x00,0x00,0x60,0x90,0x90,0xFE,0x00,0x00, //d
            // 0x00,0x00,0x1C,0x2A,0x2A,0x2A,0x24,0x00, //e
            // 0x00,0x00,0x1C,0x2A,0x2A,0x2A,0x24,0x00, //e
            // 0x00,0x00,0x7E,0x12,0x12,0x0C,0x00,0x00, //p
            // 0x00,0x00,0x08,0x7E,0x88,0x40,0x00,0x00, //t
            // 0x3C,0x42,0x95,0xB1,0xB1,0x95,0x42,0x3C, //:)
            0x80, 0x40, 0x20, 0x10, 0x20, 0x02, 0x12, 0x0e,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,  //NULL
        ];
        let tab = [0xfe,0xfd,0xfb,0xf7,0xef,0xdf,0xbf,0x7f];
        self.display_data(data, tab);
    }
}