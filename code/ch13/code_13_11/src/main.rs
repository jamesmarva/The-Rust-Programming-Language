use std::thread;
use std::time::Duration;

fn main() {

    let simulate_intensity = 11;
    let simulate_random = 7;
    
    
}

fn generate_wordout(intensity: u32, random_num: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculation slowly.");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25  {
        println!("Today, do {} pushups!", expensive_result.value(intensity)); 
        println!("Next, do {} situps!", expensive_result.value(intensity)); 
    } else {
        if random_num  == 3 {
            println!("Take a break today! Remember to stay hydrated!"); 
        } else {
            println!( "Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32 
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where  
        T: Fn(u32) -> u32
{
    fn new(cal: T) -> Cacher<T> {
        Cacher{
            calculation: cal,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation) (arg);
                self.value = Some(v);
                v
            }
        }
    }
}

