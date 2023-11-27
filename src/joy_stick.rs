use crate::adc_0832::ADC;
use gpio::GpioValue::{High, Low};
use gpio::GpioIn;

const BUTTON_PIN: u16 = 24;

pub struct JoyStick {
    acd: ADC,
    button: gpio::sysfs::SysFsGpioInput,
}

impl JoyStick {
    pub fn new() -> JoyStick {
        let acd = ADC::new();
        let button = gpio::sysfs::SysFsGpioInput::open(BUTTON_PIN).expect("Pin is active");

        Self { acd, button}
    }

    pub fn output(&mut self) -> (u8, u8, u8) {
        let horizontal = self.acd.get_result(0);
        let vertical = self.acd.get_result( 1);
        let pressed = match  self.button.read_value().expect("Pin is read") {
            High => { println!("pressed"); 1 },
            Low => 0
        };
        (horizontal, vertical, pressed)
    }

}