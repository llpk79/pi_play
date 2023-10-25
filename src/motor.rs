use gpio::{GpioOut};

pub struct Motor {
    in_: gpio::sysfs::SysFsGpioOutput,
    out: gpio::sysfs::SysFsGpioOutput
}

impl Motor {
    pub fn new() -> Motor {
        let in_ = gpio::sysfs::SysFsGpioOutput::open(5).unwrap();
        let out = gpio::sysfs::SysFsGpioOutput::open(6).unwrap();
        Self {in_, out}
    }

    pub fn start_motor(&mut self) {
        self.out.set_value(true).unwrap();
        self.in_.set_value(false).unwrap();
    }

    pub fn stop_motor(&mut self) {
        self.out.set_value(true).unwrap();
        self.in_.set_value(true).unwrap();
    }
}