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
    low_power_mask: u16,
    standard_res_mask: u16,
    high_res_mask: u16,
    ultra_high_res_mask: u16,

    // Registers.
    control: u8,
    temp_data: u8,
    pressure_data: u8,

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
    mb: i16,
    mc: i16,
    md: i16,
    // Commands.
    read_temp: u16,
    read_pressure: u16,
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
        let low_power_mask = 0x00_u16;
        let standard_res_mask = 0x10_u16;
        let high_res_mask = 0x20_u16;
        let ultra_high_res_mask = 0x30_u16;
        let control = 0xF4_u8;
        let temp_data = 0xF6_u8;
        let pressure_data = 0xF6_u8;
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
        let mb = 0_i16;
        let mc = 0_i16;
        let md = 0_i16;
        let read_temp = 0x2E_u16;
        let read_pressure = 0x34_u16;

        Self {
            i2c,
            addr,
            low_power_mask,
            standard_res_mask,
            high_res_mask,
            ultra_high_res_mask,
            control,
            temp_data,
            pressure_data,
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
            mb,
            mc,
            md,
            read_temp,
            read_pressure
        }
    }

    pub fn init(&mut self) {
        self.i2c.smbus_set_slave_address(self.addr,false).expect("Slave addr should be set");
        self.ac1 = match self.i2c.smbus_read_word_data(self.cal_ac1) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
        self.ac2 = match self.i2c.smbus_read_word_data(self.cal_ac2) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
        self.ac3 = match self.i2c.smbus_read_word_data(self.cal_ac3) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
        self.ac4 = match self.i2c.smbus_read_word_data(self.cal_ac4) {
            Ok(data) => data,
            Err(_e) => panic!()
        };
        self.ac5 = match self.i2c.smbus_read_word_data(self.cal_ac5) {
            Ok(data) => data,
            Err(_e) => panic!()
        };
        self.ac6 = match self.i2c.smbus_read_word_data(self.cal_ac6) {
            Ok(data) => data,
            Err(_e) => panic!()
        };
        self.b1 = match self.i2c.smbus_read_word_data(self.cal_b1) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
        self.b2 = match self.i2c.smbus_read_word_data(self.cal_b2) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
        self.mb = match self.i2c.smbus_read_word_data(self.cal_mb) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
        self.mc = match self.i2c.smbus_read_word_data(self.cal_mc) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
        self.md = match self.i2c.smbus_read_word_data(self.cal_md) {
            Ok(data) => data as i16,
            Err(_e) => panic!()
        };
    }

    fn read_raw_temp(&mut self) -> u16 {
        self.i2c.smbus_write_word_data(self.control, self.read_temp).expect("data should write");
        thread::sleep(Duration::from_micros(5));
        return match self.i2c.smbus_read_word_data(self.temp_data) {
            Ok(raw_temp) => raw_temp,
            Err(_e) => panic!()
        }
    }

    fn read_raw_pressure(&mut self, mode: &Mode) -> u16 {
        let raw_modifier: u16;
        match mode {
            Mode::LowPower => {
                self.i2c.smbus_write_word_data(self.control, self.read_pressure + (self.low_power_mask << 6)).expect("should write");
                thread::sleep(Duration::from_micros(5));
                raw_modifier = self.low_power_mask;
            }
            Mode::Standard => {
                self.i2c.smbus_write_word_data(self.control, self.read_pressure + (self.standard_res_mask << 6)).expect("should write");
                thread::sleep(Duration::from_micros(8));
                raw_modifier = self.standard_res_mask;
            }
            Mode::HighRes => {
                self.i2c.smbus_write_word_data(self.control, self.read_pressure + (self.high_res_mask << 6)).expect("should write");
                thread::sleep(Duration::from_micros(14));
                raw_modifier = self.high_res_mask;
            }
            Mode::UltraHighRes => {
                self.i2c.smbus_write_word_data(self.control, self.read_pressure + (self.ultra_high_res_mask << 6)).expect("should write");
                thread::sleep(Duration::from_micros(26));
                raw_modifier = self.ultra_high_res_mask;
            }
        }
        let msb = match self.i2c.smbus_read_byte_data(self.pressure_data) {
            Ok(msb) => msb,
            Err(_e) => panic!()
        };
        let lsb = match self.i2c.smbus_read_byte_data(self.pressure_data + 0x10) {
            Ok(lsb) => lsb,
            Err(_e) => panic!()
        };
        let xlsb = match self.i2c.smbus_read_byte_data(self.pressure_data + 0x20) {
            Ok(xlsb) => xlsb,
            Err(_e) => panic!()
        };
        (((msb << 16) + (lsb << 8) + xlsb) >> (8 - raw_modifier)) as u16
    }

    pub fn read_temperature(&mut self) -> i16 {
        let raw_temp = self.read_raw_temp();
        // From datasheet
        let x1: i16 = (((raw_temp - self.ac6) * self.ac5) >> 15) as i16;
        let x2: i16 = (self.mc << 11) / (x1 + self.mb);
        let b5 = x1 + x2;
        return ((b5 + 8) >> 4) / 10
    }

    pub fn read_pressure(&mut self, mode: &Mode) -> i64 {
        let raw_temp = self.read_raw_temp();
        let raw_pressure = self.read_raw_pressure(mode);
        // From datasheet.
        let x1: i16 = (((raw_temp - self.ac6) * self.ac5) >> 15) as i16;
        let x2: i16 = (self.mc << 11) / (x1 + self.md);
        let b5 = x1 + x2;
        let b6 = b5 - 4000;
        let y1 = (self.b2 * (b6 * b6) >> 12) >> 11;
        let y2 = (self.ac2 * b6) >> 11;
        let y3 = y1 + y2;
        let b3 = match mode {
            Mode::LowPower => (((self.ac1 * 4 + y3) << self.low_power_mask) + 2) / 4,
            Mode::Standard => (((self.ac1 * 4 + y3) << self.standard_res_mask) + 2) / 4,
            Mode::HighRes => (((self.ac1 * 4 + y3) << self.high_res_mask) + 2) / 4,
            Mode::UltraHighRes => (((self.ac1 * 4 + y3) << self.ultra_high_res_mask) + 2) / 4,
        };
        let z1 = (self.ac3 * b6) >> 13;
        let z2 = (self.b1 * ((b6 * b6) >> 12)) >> 16;
        let z3 = ((z1 + z2) + 2) >> 2;
        let b4: i64 = ((self.ac4 as i32 * (z3 as i32 + 32768)) >> 15) as i64;
        let b7 = match mode {
            Mode::LowPower => (raw_pressure as i16 - b3) as i64 * (50_000 >> self.low_power_mask),
            Mode::Standard => (raw_pressure as i16 - b3) as i64 * (50_000 >> self.standard_res_mask),
            Mode::HighRes => (raw_pressure as i16 - b3) as i64 * (50_000 >> self.high_res_mask),
            Mode::UltraHighRes => (raw_pressure as i16 - b3) as i64 * (50_000 >> self.ultra_high_res_mask)
        };
        let pressure = match b7 < 0x80000000 {
            true => (b7 * 2) / b4,
            false => (b7 / b4) * 2,
        };
        let mut final1 = (pressure >> 8) * (pressure >> 8);
        final1 = (final1 * 3038) >> 16;
        let final2 = (-7357 * pressure) >> 16;

        pressure + ((final1 + final2 + 3791) >> 4)
    }

    pub fn read_altitude(&mut self, mode: Mode) -> f32 {
        let pressure = self.read_pressure(&mode);
        44330.0_f32 * (1.0 - f32::powf(pressure as f32 / SEA_LEVEL_PA, 1.0/5.255))
    }

    pub fn read_sea_level_pressure(&mut self, mode: Mode, altitude: f32) -> f32 {
        let pressure = self.read_pressure(&mode);
        pressure as f32 / f32::powf(1.0 - altitude / 44330.0_f32, 5.255)
    }
}