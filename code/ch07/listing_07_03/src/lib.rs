#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    
    crate::front_of_house::hosting.add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

}



