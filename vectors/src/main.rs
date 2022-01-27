fn main() {
    //when creating an empty vector, we need to specify the type of the vector
    let v: Vec<i32> = Vec::new();
    //the exception is if we immediately push an element onto the vector
    // Rust will infer the type of the vector
    let mut x = Vec::new();
    x.push(5);
    //there are two methods to reading elements from a vector
    //the first is to use the indexing operator
    let j = vec![1,2,3,4,5];
    // this gives us a reference to the element at index 2
    let third: &i32 = &j[2];
    println!("The third element is {}", third);
    //the second is to use the get method
    let j = vec![1,3,3,4,5];
    match j.get(2) {
        // this gives us an Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // rust has two ways to reference an element
    let m = vec![6,7,8,9,10];
    // this will cause the compiler to panic
    // best used when you want your program to crash if the index is out of bounds
    // let does_not_exist = &m[100];
    // this returns None and the program will continue
    let does_not_exist = m.get(100);

    // you cannot mutate a vector while holding a reference to it
    let mut k = vec![1,2,3,4,5];
    let first = &k[0];
    // this will cause a compiler error
    // k.push(6);
    println!("The first element is: {}", first);
    // why? -- adding a new element to the end of the vector might require reallocation
    // the reference might be pointing to deallocated memory
    
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.32),
    ];
}

