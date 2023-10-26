extern crate i2c_linux;

use std::fs::File;
use i2c_linux::I2c;
use std::thread;
use std::time::Duration;

pub enum Func {
    I2c,
    SmbusPec,
    SmbusQuick,
    SmbusReadByte,
    SmbusWriteByte,
    SmbusReadByteData,
    SmbusWriteByteData,
    SmbusReadWordData,
    SmbusWriteWordData,
    SmbusProcCall,
    SmbusWriteBlockData,
    SmbusReadI2cBlock,
    SmbusWriteI2cBlock,
    SmbusByte,
    SmbusByteData,
    SmbusWordData,
    SmbusI2cBlock,
    SmbusEmul,
}

pub struct LCD {
    i2c: I2c<File>,
    enable_mask: u8,
    rw_mask: u8,
    rs_mask: u8,
    backlight_mask: u8,
    data_mask: u8,
    columns: u8,
    rows: u8,
    bus: u8,
    addr: u16,
}

impl LCD {
    pub fn new() -> LCD {
        let enable_mask = (1<<2) as u8;
        let rw_mask = (1<<1) as u8;
        let rs_mask = (1<<0) as u8;
        let backlight_mask = (1<<3) as u8;
        let data_mask = 0x00u8;
        let columns = 16u8;
        let rows = 2u8;
        let bus = 1u8;
        let addr = 0x27u16;
        let dev_path = "/dev/i2c-1".to_string();
        let mut i2c = I2c::from_path(dev_path).unwrap();
        Self {
            i2c,
            enable_mask,
            rw_mask,
            rs_mask,
            backlight_mask,
            data_mask,
            columns,
            rows,
            bus,
            addr,
        }
    }

    pub fn set_slave_address(&mut self) {
        self.i2c.smbus_set_slave_address(self.addr, false).unwrap();
    }

    fn write_byte_data(&mut self, data: u8) {
        self.i2c.smbus_write_byte_data(0,data).unwrap();
    }

    fn write_4_bits(&mut self, mut value: u8) {
        value = value & !self.enable_mask;
        self.write_byte_data(value);
        self.write_byte_data(value | self.enable_mask);
        self.write_byte_data(value);
    }

    pub fn display_init(&mut self) {
        thread::sleep(Duration::from_micros(1));
        self.write_4_bits(0x30);
        thread::sleep(Duration::from_micros(45));
        self.write_4_bits(0x30);
        thread::sleep(Duration::from_micros(45));
        self.write_4_bits(0x30);
        thread::sleep(Duration::from_micros(1));
        self.write_4_bits((0x20|0x88) & 0xf0);
        self.write_4_bits((0x20|0x88) <<4);
        thread::sleep(Duration::from_micros(50));
        self.write_4_bits((0x04|0x88) & 0xf0);
        self.write_4_bits((0x04|0x88) <<4);
        thread::sleep(Duration::from_micros(80));
        self.write_4_bits(0x10 & 0xf0);
        self.write_4_bits(0x10 <<4);
        self.write_4_bits((0x04|0x02) & 0xf0);
        self.write_4_bits((0x04|0x02) <<4);
    }
}

// pub fn read () {
//     let mut i2c = I2c::from_path("/dev/i2c-1").unwrap();
//     // i2c.smbus_set_slave_address(0x50, false).unwrap();
//     let data = i2c.i2c_functionality().unwrap();
//     println!("I2c data: {:?}", data);
// }