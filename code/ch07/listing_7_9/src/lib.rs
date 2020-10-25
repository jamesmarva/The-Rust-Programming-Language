mod  back_of_house {
    pub struct Breakafast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakafast {
        pub fn summer(toast: &str) -> Breakafast {
            Breakafast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakafast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please.", meal.toast);
}

#[cfg(test)]
mod tests {
    use super::eat_at_restaurant;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        eat_at_restaurant();
    }
}
