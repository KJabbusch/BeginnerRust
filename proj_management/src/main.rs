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
// unidiomatic way to bring seat_at_table into scope
// specifying the parent module when calling the function makes it clear that it is not locally defined
use crate::front_of_house::hosting::seat_at_table;

// when we bring a name into scope with use, the default is private
// re-exporting by combining use and pub
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // unclear where seat_at_table is defined, so we should specify the parent module (hosting::) like above
    seat_at_table();

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
    println!("Customer order 1: {:?} and 2: {:?}", order1, order2);
}

//however, when bringing in structs, enums, etc, it is idiomatic to specify the full path
// use std::collections::HashMap;
// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
//    }

// the exception to this standard is when bringing in two items with the same name into scope
// Rust will not allow it, so the parent module must be specified OR
// you can specify a new local name for the imported item
// use std::fmt::Result;
// use std::io::Result as IoResult;