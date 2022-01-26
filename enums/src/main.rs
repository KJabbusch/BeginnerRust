fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // we can't just return i+1, we have to wrap it in Some
        // that's because our return value is an Option<i32>
        Some(i) => Some(i + 1),
    }
}