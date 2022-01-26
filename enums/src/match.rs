fn main(){
    val_in_cents(Coin::Quarter(UsState::Alaska));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn val_in_cents(coin: Coin) -> u8 {
    // the match expression is exhaustive -- it will match every possible value of the enum
    match coin {
        // for each type of coin, we have a match arm (we return the value of the coin)
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
