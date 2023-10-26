use pi_play_lib::lcd;
// use pi_play_lib::segment::Segment;
// use std::thread;
// use std::time::Duration;
// use pi_play_lib::temp::{read_temp};
// use pi_play_lib::motor::{Motor};
// use std::str::FromStr;
// use pi_play_lib::distance::Distance;
use pi_play_lib::lcd::{LCD};

// const FAHRENHEIT: bool = false;

fn main() {
    let mut lcd = LCD::new();
    lcd.set_slave_address();
    lcd.backlight_off();
    lcd.display_init();
    lcd.backlight_on();
    // let data = vec!["I'm alive!!".to_string(), "look at me!!".to_string()];
    // lcd.display_data(data);
    // let mut segment_display = Segment::new();
    // segment_display.init();
    // let mut motor = Motor::new();
    // motor.stop();
    // let mut distance = Distance::new();
    // loop {
        // let mut temp = read_temp(FAHRENHEIT);
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
        // temp.insert(2, '.');
        // temp = temp[..5].to_string();
        // println!("Current temp: {} {}\n", temp, if FAHRENHEIT {"F"} else {"C"});

        // let measure = distance.print_measure();
        // println!("measure {}\n", measure);

        // segment_display.display_dec(measure);
        // thread::sleep(Duration::from_millis(10));
    // }
}
