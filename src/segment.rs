use gpio::{GpioOut};
use std::{thread};
use std::time::Duration;


const DIO: u16 = 13;
const CLK: u16 = 12;
const STB: u16 = 17;
const BITORDER: u8 = 1;

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

    pub fn init(self) {
        self.send_command(0x8f);
    }

    fn shift_out(mut self, mut val: u8) {
        for i in 0..8 {
            if BITORDER == 1 {
                val = val & (1 << i);
            } else {
                val = val & (1 << (7 - i));
            }
            self.dio.set_value(val)?;
            self.clk.set_value(true)?;
            thread::sleep(Duration::from_millis(10));
            self.clk.set_value(false)?;
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn send_command(mut self, cmd: u8) {
        self.stb.set_value(false)?;
        self.shift_out(cmd);
        self.stb.set_value(true)?;
    }

    pub fn display_num(mut self, num: u8) {
        let digits: Vec<i8> = vec![0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f];
        self.send_command(0x40);
        self.stb.set_value(false)?;
        self.shift_out(0xc0);
        self.shift_out(digits[(num as u16/1000)%10]);
        self.shift_out(0x00);
        self.shift_out(digits[(num/100)%10]);
        self.shift_out(0x00);
        self.shift_out(digits[(num/10)%10]);
        self.shift_out(0x00);
        self.shift_out(digits[num%10]);
        self.shift_out(0x00);
        self.stb.set_value(true)?;
    }

    pub fn display_dec(mut self, num: f32) {
        let digits = vec![0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f];
        let integer: i32;
        let decimal: i32;

        let pro: i32 = (num * 100);

        integer = pro / 100;
        decimal = pro % 100;
        self.send_command(0x40);
        self.stb.set_value(false)?;
        self.shift_out(0xc0);
        self.shift_out(digits[integer as u16/10]);
        self.shift_out(0x00);
        self.shift_out(digits[(integer%10) | 0x80]);
        self.shift_out(0x00);
        self.shift_out(digits[decimal/10]);
        self.shift_out(0x00);
        self.shift_out(digits[decimal%10]);
        self.shift_out(0x00);
        self.stb.set_value(true)?;
    }
}

