// use pi_play_lib::segment::Segment;
// use pi_play_lib::temp::read_temp;
use std::thread;
use std::time::Duration;
// use pi_play_lib::distance::Distance;
use pi_play_lib::lcd::LCD;
use pi_play_lib::temp_humid::measure_temp_humid;

fn main() {
    let mut lcd = LCD::new();
    lcd.set_slave_address();
    lcd.backlight_off();
    lcd.display_init();
    lcd.backlight_on();
    // let data = vec![
    //     "Temperature        ".to_string(),
    //     "F:      C:     ".to_string(),
    // ];
    // lcd.display_data(data);
    // let mut segment_display = Segment::new();
    // segment_display.init();
    // let mut distance = Distance::new();

    loop {
        // let mut f_temp = read_temp(true);
        // let mut c_temp = read_temp(false);

        // let measure = distance.print_measure() + "   ";
        // let data = format!("{}.{}", &measure[0..1], &measure[1..]);
        // segment_display.display_dec(measure);

        // f_temp = f_temp[..4].to_string();
        // c_temp = c_temp[..4].to_string();
        //
        // lcd.cursor_to(1, 3);
        // lcd.print_line(&f_temp);
        // lcd.cursor_to(1, 11);
        // lcd.print_line(&c_temp);
        // println!("here");
        let hum_temp = measure_temp_humid();
        lcd.display_data(hum_temp);
        thread::sleep(Duration::from_millis(2000));
        // println!("now here");
    }
}
