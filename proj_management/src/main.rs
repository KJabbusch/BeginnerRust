fn main() {
    eat_at_restaurant();
}

mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        // unlike structs, pub enums have public variants by default
        Soup,
        Salad,
    }
}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
}
// we can bring hosting module into scope with the use keyword -- absolute path
use crate::front_of_house::hosting;
// we can bring Appetizer enum into scope with the use keyword -- relative path
use self::back_of_house::Appetizer;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
    println!("Customer order 1: {:?} and 2: {:?}", order1, order2);
}

