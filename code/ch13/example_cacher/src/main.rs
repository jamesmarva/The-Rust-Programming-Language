fn main() {

    let mut cache_closure = Cacher::new(cal);
    println!("value: {}", (cache_closure.get(10)).to_string());
    
}

fn cal(val: usize) -> usize {
    val * val
}

struct Cacher<T>
where T: Fn(usize) -> usize,
{
    calculation:T,
    arg: usize,
    value: Option<usize>,
}

impl<T> Cacher<T>
where T: Fn(usize) -> usize 
{
    fn new(f: T) -> Self{
        Cacher {
            calculation: f,
            arg: 0,
            value: None,
        }
    }

    fn get(&mut self, argu: usize) -> usize {
        let mut tmp_clo = |x: usize| {
            let tmp_val = (self.calculation)(x);
            self.value = Some(tmp_val);
            self.arg = x;
            tmp_val
        };
       
        // let mut local_value = self.value;
        match self.value {
            Some(val) => {
                if self.arg == argu {
                    val
                } else {
                    tmp_clo(argu)
                }
            },
            None => {
                tmp_clo(argu)
            }
        }
    }
}
