use gpio::GpioOut;
use gpio::GpioValue::{High, Low};
use std::thread;
use std::time::Duration;

const RCLK: u16 = 17;
const SRCLK: u16 = 27;
const SDI: u16 = 22;

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

        Self { rclk, srclk, sdi }
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

    pub fn display_data(&mut self, data: &Vec<u8>, tab: [u8; 8]) {
        for i in 0..data.len() - 8 {
            for _ in 0..15 {
                for j in 0..8 {
                    self.input(data[i + j]);
                    self.input(tab[j]);
                    self.output();
                    // Scroll speed.
                    thread::sleep(Duration::from_micros(500));
                }
            }
        }
    }

    pub fn test(&mut self) {
        let data = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //NULL
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
            0x40, 0x20, 0x10, 0x08, 0x10, 0x0a, 0x06, 0x0e, // line go up
            0x02, 0x04, 0x08, 0x10, 0x08, 0x50, 0x60, 0x70, // line go down
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //NULL
        ];
        let tab = [0xfe, 0xfd, 0xfb, 0xf7, 0xef, 0xdf, 0xbf, 0x7f];
        self.display_data(&data.to_vec(), tab);
    }
}

pub struct DotMatrixData {
    pub data: Vec<Vec<u8>>,
    pub tab: [u8; 8],
    pub rev_tab: [u8; 8],
}

impl DotMatrixData {
    pub fn new() -> DotMatrixData {
        let tab = [0xfe, 0xfd, 0xfb, 0xf7, 0xef, 0xdf, 0xbf, 0x7f];
        let rev_tab = [0x7f, 0xbf, 0xdf, 0xef, 0xf7, 0xfb, 0xfd, 0xfe];
        let line_go_up = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // 0x40, 0x20, 0x10, 0x08, 0x10, 0x0a, 0x06, 0x0e, // line go up
            0x0e, 0x06, 0x0a, 0x10, 0x08, 0x10, 0x20, 0x40, 0x80, // rev line go up
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let line_go_down = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // 0x02, 0x04, 0x08, 0x10, 0x08, 0x50, 0x60, 0x70, // line go down
            0x70, 0x60, 0x50, 0x08, 0x10, 0x08, 0x04, 0x02, 0x01, // rev line go down
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let line_stay_same = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x38, 0x54, 0x10, 0x20, 0x10,
            0x08, 0x10, 0x10, 0x10, // rev line stay same
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let pressure = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x12, 0x12, 0x12, 0x0C, 0x20,
            0x54, 0x54, // Pa.
            0x54, 0x78, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let temp = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x02, 0x7E, 0x02, 0x02,
            0x00, // Tmp.
            0x7C, 0x04, 0x78, 0x04, 0x78, 0x00, 0x7C, 0x24, 0x24, 0x18, 0x00, 0x40, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let humidity = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x08, 0x08, 0x7E, 0x00, 0x3C,
            0x40, 0x40, // Hum.
            0x40, 0x3C, 0x00, 0x7C, 0x04, 0x78, 0x04, 0x78, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let i_heart_macey = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x00,
            0x0C, 0x1E, 0x3E, // I :heart: Macey
            0x7C, 0x3E, 0x1E, 0x0C, 0x00, 0x7E, 0x04, 0x08, 0x08, 0x04, 0x7E, 0x00, 0x20, 0x54,
            0x54, 0x54, 0x78, 0x00, 0x38, 0x44, 0x44, 0x44, 0x00, 0x38, 0x54, 0x54, 0x54, 0x58,
            0x00, 0x1C, 0xA0, 0xA0, 0xA0, 0x7C, 0x00, 0x5E, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42,
            0x95, 0xA1, 0xA1, 0x95, 0x42, 0x3C, //:)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let data = Vec::from([
            line_go_down,
            line_go_up,
            line_stay_same,
            pressure,
            temp,
            humidity,
            i_heart_macey,
        ]);
        Self { data, tab, rev_tab }
    }
}
