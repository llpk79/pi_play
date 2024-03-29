use pi_play_lib::barometer::{Barometer, Mode::HighRes};
use pi_play_lib::dot_matrix::{DotMatrix, DotMatrixData};
use pi_play_lib::lcd::LCD;
use pi_play_lib::temp_humid::measure_temp_humid;
use std::thread;
use std::time::Duration;

fn main() {
    let mut dot_matrix = DotMatrix::new();

    let mut lcd = LCD::new();
    lcd.display_init();

    let mut barometer = Barometer::new();
    barometer.init();
    let mode = HighRes;

    let mut prev_humidity: f32 = 0.0;
    let mut prev_pressure: i64 = 0;
    let mut prev_temp: i64 = 0;

    loop {
        let raw_c = barometer.read_raw_temp();
        let celsius = barometer.read_temperature(raw_c);
        let fahrenheit = ((celsius as f32 / 10_f32) * 9.0_f32 / 5.0) + 32.0;

        let raw_pressure = barometer.read_raw_pressure(&mode);
        let pressure = barometer.read_pressure(raw_pressure, &mode);
        let (_, humidity) = measure_temp_humid();

        let message = Vec::from([
            format!(
                "C {:.1} F {:.1}        ",
                celsius as f32 / 10_f32,
                fahrenheit
            ),
            format!(
                "B {:.1} H {:.1}        ",
                pressure as f32 / 100_f32,
                prev_humidity
            ),
        ]);
        lcd.display_data(message);

        let dot_matrix_data = DotMatrixData::new();
        if pressure > prev_pressure {
            dot_matrix.display_data(&dot_matrix_data.data[3], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[1], dot_matrix_data.rev_tab)
        } else if pressure == prev_pressure {
            dot_matrix.display_data(&dot_matrix_data.data[3], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[2], dot_matrix_data.rev_tab);
        } else {
            dot_matrix.display_data(&dot_matrix_data.data[3], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[0], dot_matrix_data.rev_tab);
        }
        if celsius > prev_temp {
            dot_matrix.display_data(&dot_matrix_data.data[4], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[1], dot_matrix_data.rev_tab);
        } else if celsius == prev_temp {
            dot_matrix.display_data(&dot_matrix_data.data[4], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[2], dot_matrix_data.rev_tab);
        } else {
            dot_matrix.display_data(&dot_matrix_data.data[4], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[0], dot_matrix_data.rev_tab);
        }
        if humidity > prev_humidity {
            dot_matrix.display_data(&dot_matrix_data.data[5], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[1], dot_matrix_data.rev_tab);
        } else if humidity != prev_humidity || humidity == 0.0 {
            dot_matrix.display_data(&dot_matrix_data.data[5], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[2], dot_matrix_data.rev_tab);
        } else {
            dot_matrix.display_data(&dot_matrix_data.data[5], dot_matrix_data.tab);
            dot_matrix.display_data(&dot_matrix_data.data[0], dot_matrix_data.rev_tab);
        }

        dot_matrix.display_data(&dot_matrix_data.data[6], dot_matrix_data.tab);
        prev_humidity = if humidity != prev_humidity && humidity != 0.0 {
            humidity
        } else {
            prev_humidity
        };
        prev_temp = if celsius != prev_temp {
            celsius
        } else {
            prev_temp
        };
        prev_pressure = if pressure != prev_pressure {
            pressure
        } else {
            prev_pressure
        };

        thread::sleep(Duration::from_secs(15))
    }
}
