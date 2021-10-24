// nested paths in use keyword
use rand::{Rng, CryptoRng, Error};
// Glob operator
use std::io::*;

// Use mod to include other file
mod national_hq;

mod resturant_front {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}
        // fn _serve_order() {}
        fn _take_payment() {}
    }
}
// use crate::resturant_front::hosting;
// use self::resturant_front::hosting;
pub use self::resturant_front::hosting;
pub fn eat_at_resturant() {
    // Absolute
    crate::resturant_front::hosting::add_to_waitlist();

    // Realative
    resturant_front::hosting::add_to_waitlist();

    // Use keyword
    hosting::add_to_waitlist();

    // Use keword for extrenal stuff
    let _secret_number = rand::thread_rng().gen_range(1..=100);
}
// ------------------------------------------------------------
fn _serve_order() {}
mod resturant_back {
    fn _cook_order() {}

    fn _fix_incorrect_order() {
        _cook_order();
        super::_serve_order();
    }

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        _Soup,
        _Salad,
    }
}

pub fn eat_at_resturant_in_summer() {
    let mut meal = resturant_back::Breakfast::summer("rye (disgusting)");

    meal.toast = String::from("Wheat");
}

pub fn eat_appetizer_at_resturant() {
    let _order1 = resturant_back::Appetizer::_Soup;
    let _order2 = resturant_back::Appetizer::_Salad;
}