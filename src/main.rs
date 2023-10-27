// use pi_play_lib::segment::Segment;
use std::thread;
use std::time::Duration;
use pi_play_lib::temp::{read_temp};
// use pi_play_lib::motor::{Motor};
// use std::str::FromStr;
// use pi_play_lib::distance::Distance;
use pi_play_lib::lcd::LCD;

// const FAHRENHEIT: bool = false;

fn main() {
    let mut lcd = LCD::new();
    lcd.set_slave_address();
    lcd.backlight_off();
    lcd.display_init();
    lcd.backlight_on();
    let data = vec!["Temperature        ".to_string(), "F:      C:     ".to_string()];
    lcd.display_data(data);
    // let mut segment_display = Segment::new();
    // segment_display.init();
    // let mut motor = Motor::new();
    // motor.stop();
    // let mut distance = Distance::new();
    loop {
        let mut f_temp = read_temp(true);
        let mut c_temp = read_temp(false);
        // let temp_dif = i32::from_str(&temp).unwrap()- 21500i32;
        // match temp_dif > 0 {
        //     true => {
        //         let speed: u8 = (temp_dif / 255) as u8;
        //         println!("speed {}", speed);
        //         motor.run(speed);
        //         segment_display.display_help();
        //     }
        //     false => {
        //         motor.stop();
        //         segment_display.display_dec(temp.clone());
        //     }
        // }
        // f_temp.insert(2, '.');
        f_temp = f_temp[..4].to_string();

        // c_temp.insert(2, '.');
        c_temp = c_temp[..4].to_string();
        // println!("Current temp: {} {}\n", temp, if FAHRENHEIT {"F"} else {"C"});

        // let measure = distance.print_measure() + "   ";
        // let data = format!("{}.{}", &measure[0..1], &measure[1..]);
        lcd.cursor_to(1, 3);
        lcd.print_line(&f_temp);
        lcd.cursor_to(1, 11);
        lcd.print_line(&c_temp);
        // segment_display.display_dec(measure);
        thread::sleep(Duration::from_millis(1));
    }
}
