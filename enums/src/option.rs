fn main() {
    // Rust does not have null values, but it does have the Option<T> type.
    enum Option<T> {
        Some(T), // stores some value -- T being a generic type
        None, // stores no value
        // if you have a "null" value, you wrap it in the Option enum
        // this allows the type system to enforce that we handle the none case when a value does not exist
        // and guarantee that in the some case, our value exists
    }
// examples of optional values:
    // rust can infer the type based on what is passed in
    let some_num: Option<i32> = Option::Some(5);
    let some_str: Option<&str> = Option::Some("a string");
    
    let absent_num: Option<i32> = Option::None;

    // you cannot add an optional type to a different type
    // you would need to extract the value out of the Option type
    let x: i18 = 5;
    let y: Option<i32> = Option::Some(5);
    // code like this will NOT compile--- let sum: <i8 as Add<Option<i8>>>::Output = x + y;
    // the below code words even if y was None because we add a default value of 0 to handle the None case
    let sum: i8 = x + y.unwrap_or(default: 0);

}
