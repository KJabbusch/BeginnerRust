fn main() {
    let mut s = String::from("hello");
    let s2 = ", world!";
    s.push_str(s2);
    println!("{}", s);

    let mut s3 = String::from("hello, world");
    let s4 = '!';
    // we can use push for single chars
    s3.push(s4);
    println!("{}", s3);

    let s5 = String::from("hello, ");
    let s6 = String::from("world");
    let s7 = String::from("!");
    let s8 = s5 + &s6 + &s7;
    println!("{}", s8);
    // we move ownership of s5 to s7
    // we only pass a reference of s6 and s7 when we add them to s5, so ownership is not moved
    println!("We can still print the second string ({}) and third string ({}), but not the first", s6, s7);
    // note: you can ONLY add a &str to a String -- we can't add two String values together
    // even though &s6 is a &String, the compiler can coerce it to a &str
    // the add method uses a deref coercion

    // the format macro
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // the format macro is similar to println, but it returns a String
    let s9 = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", s9);

    // Indexing Strings -- Rust strings don't support indexing -- won't compile
    // theoretically would return byte values instead (e.g. &"hello"[0] would return 104, not h)
    // Rust provides different ways of intepreting raw string data so we can decide how to interpret it
    // additionally, indexing operations are expected to run in constant time -- not guaranteed w/ Rust Strings

    // however, we can slice strings, but this isn't perfect
    let hello = "Здравствуйте";
    let s10 = &hello[0..4];
    // if we wrote [0..3], rust panicks because it's not a "char boundary" 
    println!("{}", s10);

    // iterating over strings to get individual characters
    for char in "नमस्ते".chars() {
        println!("{}", char);
    }

    // iterating over strings to get individual bytes
    for byte in "meep".bytes() {
        println!("{}", byte);
    }

    

}
