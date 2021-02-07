use std::fmt::{Display, Formatter, Result};

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("james")]);
    println!("w = {}", w);

    let wrap = WrapperVectorUsize(vec![1, 2,3, 4]);
    let v1 = vec![1, 3, 4, 45, 5, 5,5 ];
    let v2: Vec<Vec<usize>> = vec!(vec!(1, 2, 4, 5), vec!(44, 55, 66));
    
    let v3: [[u32; 2]; 2] = [[1,3],[4, 5]];
    let w1 = WarapperArrayU32(v3);
    println!("{}", w1);
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct WrapperVectorUsize(Vec<usize>);

struct WarapperArrayU32([[u32; 2]; 2]);

impl Display for WarapperArrayU32 {
    fn fmt (&self, f: &mut Formatter) -> Result {
        let mut rst = String::new();
        let idx = 0;
        // 
        for &row_data in self.0.iter() {
            rst.push_str("[");
            for &item in row_data.iter() {
                rst.push_str(&item.to_string());
                rst.push_str(", ");
            }
            rst.push_str("]");
        }
        write!(f, "{}", rst)
    }
}

fn array_iter() {
    let a = [1, 2, 3, 4, 6];
    for i in a.iter() {
        println!("{}", i.to_string());        
    }
}

fn vector_iter() {
    let v1: Vec<usize> = vec![1, 2, 3, 4];
    
    for &item in v1.iter() {
        
    }
}


// impl Display  for WrapperVectorUsize {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "[{}]", self.0.concat())
//     }
// }

