pub mod front_of_houst {
    pub mod hosting { 
        pub fn add_to_waitlist() {
            println!("{}", "asdfasfasfas");
        }
    }
}
#[test]
pub fn eat_at_restaurant() {

    crate::front_of_houst::hosting::add_to_waitlist();

    front_of_houst::hosting::add_to_waitlist();

}





