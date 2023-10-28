use gpio::GpioValue::{High, Low};
use gpio::{GpioIn, GpioOut};
use std::thread;
use std::time::Duration;


pub struct TempHumid {
    data_pin: gpio::sysfs::SysFsGpioInput,
}

impl TempHumid {
    pub fn new() -> TempHumid {
        let mut start_pin = gpio::sysfs::SysFsGpioOutput::open(18).unwrap();
        start_pin.set_value(false).unwrap();
        thread::sleep(Duration::from_millis(20));
        start_pin.set_value(true).unwrap();
        let mut data_pin = gpio::sysfs::SysFsGpioInput::open(18).unwrap();
        while data_pin.read_value().unwrap() == Low {
            continue
        };
        while data_pin.read_value().unwrap() == High {
            continue
        };
        println!("data pin {:?}\n", data_pin);
        Self { data_pin }
    }

    fn read(&mut self) -> Vec<i32> {
        let mut j = 0;
        let mut data = Vec::new();
        while j < 40 {
            let mut k = 0;
            println!("data pin read{:?}\n", self.data_pin);
            while self.data_pin.read_value().unwrap() == Low {
                continue;
            }
            while self.data_pin.read_value().unwrap() == High {
                k += 1;
                if k > 100 {
                    break
                }
            }
            if k < 8 {
                data.push(0);
            } else {
                data.push(1)
            }
            j += 1;
        }
        data
    }

    fn translate(&mut self) -> (i32, i32, i32, i32) {
        let data = self.read();

        let hum_bit = Vec::from(&data[0..8]);
        let hum_dec_bit = Vec::from(&data[8..16]);
        let temp_bit = Vec::from(&data[16..24]);
        let temp_dec_bit = Vec::from(&data[24..32]);
        let check_bit = Vec::from(&data[32..40]);
        let mut hum = 0;
        let mut hum_dec = 0;
        let mut temp = 0;
        let mut temp_dec = 0;
        let mut check = 0;

        for i in 0..8 {
            hum += hum_bit[i] * (2 << (7 - i));
            hum_dec += hum_dec_bit[i] * (2 << (7 - 1));
            temp += temp_bit[i] * (2 << (7 - i));
            temp_dec += temp_dec_bit[i] * (2 << (7 - i));
            check += check_bit[i] * (2 << (7 - i));
        }
        if check != hum + hum_dec + temp + temp_dec {
            println!("Error reading temp/humidity");
            // self.translate();
        }
        (hum, hum_dec, temp, temp_dec)
    }

    pub fn string_data(&mut self) -> Vec<String> {
        let (hum, hum_dec, temp, temp_dec) = self.translate();
        let hum = format!("Humidity: {}.{}", hum, hum_dec);
        let temp = format!("C: {}.{}", temp, temp_dec);
        vec![hum, temp]
    }
}