use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::thread;
use std::time::Duration;


const CS_PIN: u16 = 16;
const CLK_PIN: u16 = 20;
const DIO_PIN: u16 = 21;

pub struct ADC {
    cs: gpio::sysfs::SysFsGpioOutput,
    clk: gpio::sysfs::SysFsGpioOutput,
}

impl ADC {
    pub fn new() -> ADC {
        let cs = gpio::sysfs::SysFsGpioOutput::open(CS_PIN).expect("Pin should be active");
        let clk = gpio::sysfs::SysFsGpioOutput::open(CLK_PIN).expect("Pin should be active");

        Self { cs, clk }
    }

    pub fn get_result(&mut self, channel: u8) -> u8 {
        let mut data_out = gpio::sysfs::SysFsGpioOutput::open(DIO_PIN).expect("Pin should be active");
        self.cs.set_value(Low).expect("Pin should set");

        self.clk.set_value(Low).expect("Pin should set");
        data_out.set_value(High).expect("Pin should set");
        thread::sleep(Duration::from_micros(2));
        self.clk.set_value(High).expect("Pin should set");
        thread::sleep(Duration::from_micros(2));
        self.clk.set_value(Low).expect("Pin should set");

        data_out.set_value(High).expect("Pin should set");
        thread::sleep(Duration::from_micros(2));
        self.clk.set_value(High).expect("Pin should set");
        thread::sleep(Duration::from_micros(2));
        self.clk.set_value(Low).expect("Pin should set");

        match channel {
            0 => data_out.set_value(Low).expect("Pin should set"),
            1 => data_out.set_value(High).expect("Pin should set"),
            _ => panic!()
        }
        thread::sleep(Duration::from_micros(2));

        self.clk.set_value(High).expect("Pin should set");
        data_out.set_value(High).expect("Pin should set");
        thread::sleep(Duration::from_micros(2));
        self.clk.set_value(Low).expect("Pin should set");
        data_out.set_value(High).expect("Pin should set");
        thread::sleep(Duration::from_micros(2));

        let mut lsb_data: u8 = 0;
        let mut data_in = gpio::sysfs::SysFsGpioInput::open(DIO_PIN).expect("Pin is active");
        for _ in 0..8 {
            self.clk.set_value(High).expect("Pin should set");
            thread::sleep(Duration::from_micros(2));
            self.clk.set_value(Low).expect("Pin should set");
            thread::sleep(Duration::from_micros(2));
            match data_in.read_value().expect("Pin should read") {
                High => lsb_data = lsb_data << 1 | 255,
                Low => lsb_data = lsb_data << 1 | 0
            }
        }
        let mut msb_data: u8 = 0;
        for i in 0..8 {
            match data_in.read_value().expect("Pin should read") {
                High => msb_data = msb_data | 255 << i,
                Low => msb_data = msb_data | 0 << i,
            };
            self.clk.set_value(High).expect("Pin should set");
            thread::sleep(Duration::from_micros(2));
            self.clk.set_value(Low).expect("Pin should set");
            thread::sleep(Duration::from_micros(2));
        }
        self.cs.set_value(High).expect("Pin should set");

        if lsb_data == msb_data {
            lsb_data
        }
        else {
            0
        }
    }
}