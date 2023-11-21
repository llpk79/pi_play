extern crate i2c_linux;

use i2c_linux::I2c;
use std::fs::File;
use std::thread;
use std::time::Duration;

const SEA_LEVEL_PA: f32 = 101_325.0;

pub struct Barometer {
    // Device.
    i2c: I2c<File>,

    // Default address.
    addr: u16,

    // Operating modes.
    low_power_mask: u8,
    standard_res_mask: u8,
    high_res_mask: u8,
    ultra_high_res_mask: u8,

    // Registers.
    control: u8,
    msb: u8,
    lsb: u8,
    xlsb: u8,

    // Calibration registers.
    cal_ac1: u8,
    cal_ac2: u8,
    cal_ac3: u8,
    cal_ac4: u8,
    cal_ac5: u8,
    cal_ac6: u8,
    cal_b1: u8,
    cal_b2: u8,
    cal_mb: u8,
    cal_mc: u8,
    cal_md: u8,

    // Registers
    ac1: i16,
    ac2: i16,
    ac3: i16,
    ac4: u16,
    ac5: u16,
    ac6: u16,
    b1: i16,
    b2: i16,
    b5: i64,
    mb: i16,
    mc: i16,
    md: i16,
    // Commands.
    read_temp: u8,
    read_pressure: u8,
}

pub enum Mode {
    LowPower,
    Standard,
    HighRes,
    UltraHighRes,
}

impl Barometer {
    pub fn new() -> Barometer {
        let i2c = I2c::from_path("/dev/i2c-1".to_string()).expect("Device should be found");
        let addr = 0x77_u16;
        let low_power_mask = 0x00_u8;
        let standard_res_mask = 0x10_u8;
        let high_res_mask = 0x20_u8;
        let ultra_high_res_mask = 0x30_u8;
        let control = 0xF4_u8;
        let msb = 0xF6_u8;
        let lsb = 0xF7_u8;
        let xlsb = 0xF8_u8;
        let cal_ac1 = 0xAA_u8;
        let cal_ac2 = 0xAC_u8;
        let cal_ac3 = 0xAE_u8;
        let cal_ac4 = 0xB0_u8;
        let cal_ac5 = 0xB2_u8;
        let cal_ac6 = 0xB4_u8;
        let cal_b1 = 0xB6_u8;
        let cal_b2 = 0xB8_u8;
        let cal_mb = 0xBA_u8;
        let cal_mc = 0xBC_u8;
        let cal_md = 0xBE_u8;
        let ac1 = 0_i16;
        let ac2 = 0_i16;
        let ac3 = 0_i16;
        let ac4 = 0_u16;
        let ac5 = 0_u16;
        let ac6 = 0_u16;
        let b1 = 0_i16;
        let b2 = 0_i16;
        let b5 = 0_i64;
        let mb = 0_i16;
        let mc = 0_i16;
        let md = 0_i16;
        let read_temp = 0x2E_u8;
        let read_pressure = 0x34_u8;

        Self {
            i2c,
            addr,
            low_power_mask,
            standard_res_mask,
            high_res_mask,
            ultra_high_res_mask,
            control,
            lsb,
            msb,
            xlsb,
            cal_ac1,
            cal_ac2,
            cal_ac3,
            cal_ac4,
            cal_ac5,
            cal_ac6,
            cal_b1,
            cal_b2,
            cal_mb,
            cal_mc,
            cal_md,
            ac1,
            ac2,
            ac3,
            ac4,
            ac5,
            ac6,
            b1,
            b2,
            b5,
            mb,
            mc,
            md,
            read_temp,
            read_pressure
        }
    }

    fn read_u16(&mut self, command: u8) -> u16 {
        let data: u16 = match self.i2c.smbus_read_word_data(command) {
            Ok(data) => {
                let mut data = data & 0xFFFF_u16;
                data = ((data << 8) & 0xFF00) + (data >> 8);
                data
            },
            Err(_e) => panic!()
        };
        data
    }

    fn read_s16(&mut self, command: u8) -> i16 {
        let raw_read = self.read_u16(command);
        return match raw_read > 32_767 {
            true => (raw_read as i32 - 65_536_i32) as i16,
            false => raw_read  as i16
        };
    }

    pub fn init(&mut self) {
        self.i2c.smbus_set_slave_address(self.addr,false).expect("Slave addr should be set");
        // Calibration
        for i in 0..1 {
            self.ac1 = self.read_s16(self.cal_ac1 + i);
            self.ac2 = self.read_s16(self.cal_ac2 + i);
            self.ac3 = self.read_s16(self.cal_ac3 + i);
            self.ac4 = self.read_u16(self.cal_ac4 + i);
            self.ac5 = self.read_u16(self.cal_ac5 + i);
            self.ac6 = self.read_u16(self.cal_ac6 + i);
            self.b1 = self.read_s16(self.cal_b1 + i);
            self.b2 = self.read_s16(self.cal_b2 + i);
            self.mb = self.read_s16(self.cal_mb + i);
            self.mc = self.read_s16(self.cal_mc + i);
            self.md =self.read_s16(self.cal_md + i);
            println!("Calibration:\nac1 {}\nac2 {}\nac3 {}\nac4 {}\nac5 {}\nac6 {}\nb1 {}\nb2 {}\nmb {}\nmc {}\nmd {}",
                     self.ac1, self.ac2, self.ac3, self.ac4, self.ac5, self.ac6, self.b1, self.b2, self.mb, self.mc, self.md)

        }
    }

    pub fn read_raw_temp(&mut self) -> i64 {
        self.i2c.smbus_write_byte_data(self.control, self.read_temp & 0xFF).expect("data should write");
        thread::sleep(Duration::from_micros(5));
        let msb =  match self.i2c.smbus_read_byte_data(self.msb) {
            Ok(msb) => msb & 0xFF,
            Err(_e) => panic!()
        };
        // let lsb = match self.i2c.smbus_read_byte_data(self.lsb) {
        //     Ok(lsb) => lsb & 0xFF,
        //     Err(_e) => panic!()
        // };
        // ((msb as i64) << 8) + lsb as i64
        msb as i64
    }

    pub fn read_temperature(&mut self, raw_temp: i64) -> i64 {
        println!("raw temp {}", raw_temp);
        // From datasheet
        let x1: i64 = (raw_temp - self.ac6 as i64) * (self.ac5 as i64 >> 15);
        let x2: i64 = ((self.mc as i64) << 11) / (x1 + self.md as i64);
        self.b5 = x1 + x2;
        (self.b5 + 8) >> 4
    }

    pub fn read_raw_pressure(&mut self, mode: &Mode) -> i64 {
        let raw_modifier: u8;
        match mode {
            Mode::LowPower => {
                self.i2c.smbus_write_byte_data(self.control, self.read_pressure + (self.low_power_mask << 6) & 0xFF).expect("should write");
                thread::sleep(Duration::from_micros(5));
                raw_modifier = self.low_power_mask;
            }
            Mode::Standard => {
                self.i2c.smbus_write_byte_data(self.control, self.read_pressure + (self.standard_res_mask << 6) & 0xFF).expect("should write");
                thread::sleep(Duration::from_micros(8));
                raw_modifier = self.standard_res_mask;
            }
            Mode::HighRes => {
                self.i2c.smbus_write_byte_data(self.control, self.read_pressure + (self.high_res_mask << 6) & 0xFF).expect("should write");
                thread::sleep(Duration::from_micros(14));
                raw_modifier = self.high_res_mask;
            }
            Mode::UltraHighRes => {
                self.i2c.smbus_write_byte_data(self.control, self.read_pressure + (self.ultra_high_res_mask << 6) & 0xFF).expect("should write");
                thread::sleep(Duration::from_micros(26));
                raw_modifier = self.ultra_high_res_mask;
            }
        }
        let msb = match self.i2c.smbus_read_byte_data(self.msb) {
            Ok(msb) => msb & 0xFF,
            Err(_e) => panic!()
        };
        let lsb = match self.i2c.smbus_read_byte_data(self.lsb) {
            Ok(lsb) => lsb & 0xFF,
            Err(_e) => panic!()
        };
        let xlsb = match self.i2c.smbus_read_byte_data(self.xlsb) {
            Ok(xlsb) => xlsb & 0xFF,
            Err(_e) => panic!()
        };
        (((msb as i64) << 16) + ((lsb as i64) << 8) + xlsb as i64) >> (8 - raw_modifier)
    }

    pub fn read_pressure(&mut self, raw_pressure: i64, mode: &Mode) -> i64 {
        println!("raw pressure {}", raw_pressure);
        // From datasheet.
        let b6: i64 = self.b5 - 4000;
        let x1: i64 = (self.b2 as i64 * (b6 * (b6 >> 12))) >> 11;
        let x2: i64 = self.ac2 as i64 * (b6 >> 12);
        let x3: i64 = x1 + x2;
        let b3: i64 = match  mode {
            Mode::LowPower => (((self.ac1 as i64 * 4) + x3) << (self.low_power_mask + 2)) / 4,
            Mode::Standard => (((self.ac1 as i64 * 4) + x3) << self.standard_res_mask + 2) / 4,
            Mode::HighRes => (((self.ac1 as i64 * 4) + x3) << self.high_res_mask + 2) / 4,
            Mode::UltraHighRes => (((self.ac1 as i64) * 4 + x3) << self.ultra_high_res_mask + 2) / 4,
        };
        let z1: i64 = self.ac3 as i64 * (b6 >> 13);
        let z2: i64 = (self.b1 as i64 * ((b6 * b6) >> 12)) >> 16;
        let z3: i64 = ((z1 + z2) + 2) >> 2;
        let b4: u64 = self.ac4 as u64 * ((z3 as u64 + 32_768) >> 15);
        let b7: u64 = match mode {
            Mode::LowPower => (raw_pressure - b3) * (50_000 >> self.low_power_mask),
            Mode::Standard => (raw_pressure - b3) * (50_000 >> self.standard_res_mask),
            Mode::HighRes => (raw_pressure - b3) * (50_000 >> self.high_res_mask),
            Mode::UltraHighRes => (raw_pressure - b3) * (50_000 >> self.ultra_high_res_mask)
        } as u64;
        let pressure: i64 = match b7 < 0x80_000_000 {
            true => (b7 * 2) / b4,
            false => (b7 / b4) * 2,
        } as i64;
        let mut final1 = (pressure >> 8) * (pressure >> 8);
        final1 = (final1 * 3038) >> 16;
        let final2 = (-7357 * pressure) >> 16;

        pressure + (final1 + final2 + 3791) >> 4
    }

    pub fn read_altitude(&mut self, mode: Mode) -> f32 {
        let raw_pressure: i64 = self.read_raw_pressure(&mode);
        let pressure: i64 = self.read_pressure(raw_pressure, &mode);
        44330.0_f32 * (1.0 - f32::powf(pressure as f32 / SEA_LEVEL_PA, 1.0/5.255))
    }

    pub fn read_sea_level_pressure(&mut self, mode: Mode, altitude: f32) -> f32 {
        let raw_pressure: i64 = self.read_raw_pressure(&mode);
        let pressure: i64 = self.read_pressure(raw_pressure, &mode);
        pressure as f32 / f32::powf(1.0 - altitude / 44330.0_f32, 5.255)
    }
}