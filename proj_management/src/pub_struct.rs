fn main() {
    
}
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
}
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("rye");
    // even though meal is mutable, the Breakfast struct has private fields (seasonal_fruit)
    meal.toast = String::from("wheat");
    // the following code will not compile bc seasonal_fruit is a private field
    // meal.seasonal_fruit = String::from("blueberries");
    // you actually can't even read the value of seasonal_fruit too lol
    println!("The customer got {} toast and fruit_would_be_here_if_pub", meal.toast); //meal.seasonal_fruit);
}

