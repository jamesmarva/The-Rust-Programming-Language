use std::thread;
use std::time::Duration;

fn main() {
    let simlated_user_specified_value = 10;
    let simlated_random_number = 7;
    generate_workout(simlated_user_specified_value, simlated_random_number);
}

fn simlated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly.");
    thread::sleep(Duration::from_secs(1));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simlated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            )
        }
    }
}