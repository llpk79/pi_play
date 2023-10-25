use gpio::{GpioOut};
use std::{thread};
use std::time::Duration;
use std::str::FromStr;


const DIO: u16 = 27;
const CLK: u16 = 18;
const STB: u16 = 17;
const BIT_ORDER: u8 = 0;

#[derive(Debug)]
pub struct Segment {
    dio: gpio::sysfs::SysFsGpioOutput,
    clk: gpio::sysfs::SysFsGpioOutput,
    stb: gpio::sysfs::SysFsGpioOutput,
}

impl Segment {
    pub fn new() -> Segment {
        let dio = gpio::sysfs::SysFsGpioOutput::open(DIO).unwrap();
        let clk = gpio::sysfs::SysFsGpioOutput::open(CLK).unwrap();
        let stb = gpio::sysfs::SysFsGpioOutput::open(STB).unwrap();
        Self { dio, clk, stb }
    }

    pub fn init(&mut self) {
        self.send_command(0x8f);
    }

    fn shift_out(&mut self, val: u8) {
        for i in 0..8 {
            if BIT_ORDER == 0 {
                self.dio.set_value(val & (1 << i)).unwrap();
            } else {
                self.dio.set_value(val & (1 << (7 - i))).unwrap();
            }
            self.clk.set_value(true).unwrap();
            thread::sleep(Duration::from_micros(1));
            self.clk.set_value(false).unwrap();
            thread::sleep(Duration::from_micros(1));
        }
    }

    fn send_command(&mut self, cmd: u8) {
        self.stb.set_value(false).unwrap();
        self.shift_out(cmd);
        self.stb.set_value(true).unwrap();
    }

    pub fn display_int(&mut self, num: i32) {
        let digits: Vec<i8> = vec![0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f];
        self.send_command(0x40);
        self.stb.set_value(false).unwrap();
        self.shift_out(0xc0);
        self.shift_out(digits[((num / 1000) % 10) as usize] as u8);
        self.shift_out(0x00);
        self.shift_out(digits[((num/100)%10) as usize] as u8);
        self.shift_out(0x00);
        self.shift_out(digits[((num/10)%10) as usize] as u8);
        self.shift_out(0x00);
        self.shift_out(digits[(num%10) as usize] as u8);
        self.shift_out(0x00);
        self.stb.set_value(true).unwrap();
    }

    pub fn display_dec(&mut self, num: String) {
        let digits = vec![0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f];

        let integer = i32::from_str(&num[..=1]).unwrap();
        let decimal = i32::from_str(&num[2..=3]).unwrap();
        self.send_command(0x40);
        self.stb.set_value(false).unwrap();
        self.shift_out(0xc0);
        self.shift_out(digits[(integer/10) as usize]);
        self.shift_out(0x00);
        self.shift_out(digits[(integer%10) as usize] | 0x80);
        self.shift_out(0x00);
        self.shift_out(digits[(decimal/10) as usize]);
        self.shift_out(0x00);
        self.shift_out(digits[(decimal%10) as usize]);
        self.shift_out(0x00);
        self.stb.set_value(true).unwrap();
    }

    pub fn display_err(&mut self) {
        self.send_command(0x40);
        self.stb.set_value(false).unwrap();
        self.shift_out(0xc0);
        self.shift_out(0xf9); // E
        self.shift_out(0x00);
        self.shift_out(0xf9); // E
        self.shift_out(0x00);
        self.shift_out(0xf9); // E
        self.shift_out(0x00);
        self.shift_out(0xf9); // E
        self.shift_out(0x00);
        self.stb.set_value(true).unwrap();
    }

    pub fn display_paul(&mut self) {
        self.send_command(0x40);
        self.stb.set_value(false).unwrap();
        self.shift_out(0xc0);
        self.shift_out(0xf3); // P
        self.shift_out(0x00);
        self.shift_out(0xf7); // A
        self.shift_out(0x00);
        self.shift_out(0xbe); // U
        self.shift_out(0x00);
        self.shift_out(0xb8); // L
        self.shift_out(0x00);
        self.stb.set_value(true).unwrap();
    }

    pub fn display_help(&mut self) {
        self.send_command(0x40);
        self.stb.set_value(false).unwrap();
        self.shift_out(0xc0);
        self.shift_out(0x76); // H
        self.shift_out(0x00);
        self.shift_out(0x79); // E
        self.shift_out(0x00);
        self.shift_out(0x38); // L
        self.shift_out(0x00);
        self.shift_out(0x73); // P
        self.shift_out(0x00);
        self.stb.set_value(true).unwrap();
    }
}

