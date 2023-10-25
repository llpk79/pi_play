use gpio::GpioOut;

pub struct Motor {
    in_: gpio::sysfs::SysFsGpioOutput,
    out: gpio::sysfs::SysFsGpioOutput,
}

impl Motor {
    pub fn new() -> Motor {
        let mut in_ = gpio::sysfs::SysFsGpioOutput::open(5).unwrap();
        let mut out = gpio::sysfs::SysFsGpioOutput::open(6).unwrap();
        in_.set_value(false).unwrap();
        out.set_value(false).unwrap();
        Self { in_, out }
    }

    pub fn start(&mut self) {
        self.in_.set_value(true).unwrap();
        self.out.set_value(false).unwrap();
    }

    pub fn stop(&mut self) {
        self.in_.set_value(false).unwrap();
        self.out.set_value(false).unwrap();
    }

    pub fn run(&mut self, speed: u8) {
        self.in_.set_value(speed).unwrap();
        self.out.set_value(false).unwrap();
    }
}
