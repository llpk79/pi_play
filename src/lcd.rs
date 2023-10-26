extern crate i2c_linux;
use i2c_linux::I2c;

pub fn read () {
    let mut i2c = I2c::from_path("/dev/i2c-1").unwrap();
    // i2c.smbus_set_slave_address(0x50, false).unwrap();
    let data = i2c.i2c_functionality().unwrap();
    println!("I2c data: {:?}", data);
}