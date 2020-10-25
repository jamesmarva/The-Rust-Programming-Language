fn serve_order() {
    println!("serve order.");
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();

    }
    
    fn cook_order() {
        println!("cook order");
    }
}