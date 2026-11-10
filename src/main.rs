use pi_play_lib::distance::Distance;
use std::thread;
use std::time::Duration;

fn main() {
    let mut distance = Distance::new();
    println!("Here we go!");
    loop {
        let measurement = distance.print_measure();
        println!("Measurement {measurement}");
        thread::sleep(Duration::from_secs(1));
    }
}