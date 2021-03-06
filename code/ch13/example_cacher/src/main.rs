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
        let mut tmp_clo = || {
            let tmp_val = ((&self).calculation)(argu);
            self.value = Some(tmp_val);
            self.arg = argu;
            tmp_val
        };
       
        // let mut local_value = self.value;
        match self.value {
            Some(val) => {
                if self.arg == argu {
                    val
                } else {
                    let tmp_val = ((&self).calculation)(argu);
                    self.value = Some(tmp_val);
                    self.arg = argu;
                    tmp_val
                }
            },
            None => {
                let tmp_val = ((&self).calculation)(argu);
                self.value = Some(tmp_val);
                self.arg = argu;
                tmp_val
            }
        }
    }
}
