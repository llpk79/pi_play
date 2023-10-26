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
        self.i2c.smbus_write_byte_data(self.addr as u8, data | self.data_mask).unwrap();
    }

    fn write_4_bits(&mut self, mut value: u8) {
        value = value & !self.enable_mask;
        self.write_byte_data(value);
        self.write_byte_data(value | self.enable_mask);
        self.write_byte_data(value);
    }

    fn send(&mut self, data: u8) {
        self.write_4_bits(data & 0xf0);
        self.write_4_bits(data << 4);
    }

    fn command(&mut self, value: u8, delay: u64) {
        self.send(value);
        thread::sleep(Duration::from_micros(delay))
    }

    fn clear(&mut self) {
        self.command(0x10, 50u64);
    }

    pub fn display_init(&mut self) {
        thread::sleep(Duration::from_micros(10000));
        self.write_4_bits(0x30);
        thread::sleep(Duration::from_micros(45000));
        self.write_4_bits(0x30);
        thread::sleep(Duration::from_micros(45000));
        self.write_4_bits(0x30);
        thread::sleep(Duration::from_micros(15));
        self.write_4_bits(0x20);
        self.command(0x20|0x08, 50u64);
        self.command(0x04|0x08, 80u64);
        self.clear();
        self.command(0x04|0x02, 50u64);
        thread::sleep(Duration::from_micros(30000));
    }

    pub fn backlight_on(&mut self) {
        self.data_mask = self.data_mask | self.backlight_mask;
    }

    pub fn backlight_off(&mut self) {
        self.data_mask = self.data_mask & !self.backlight_mask;
    }
}

// pub fn read () {
//     let mut i2c = I2c::from_path("/dev/i2c-1").unwrap();
//     // i2c.smbus_set_slave_address(0x50, false).unwrap();
//     let data = i2c.i2c_functionality().unwrap();
//     println!("I2c data: {:?}", data);
// }