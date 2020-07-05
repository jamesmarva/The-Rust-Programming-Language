// #[cfg(test)]
// mod tests {
    
//     fn it_works() {
//         println!("{}", String::from("asdfasdfasfdasdf"));
//         println!("+++++++++++++++ {}", "asdfasdfasfdasdf");
//         println!("+++++++++++++++");

        
//     }
// }
//#[cfg(test)]
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            println!("--------------");
            assert_eq!(2 + 2, 5);
        }
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }   
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
    
}