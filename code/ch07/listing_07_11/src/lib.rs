#[cfg(test)]
mod tests {
    
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add success.");
        }
    }
}

use crate::front_of_house::hosting;
#[test]
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

}

fn main() {    
}





