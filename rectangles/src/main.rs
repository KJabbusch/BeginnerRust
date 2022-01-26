// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectange is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// REFACTORED

// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectange is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1 // tuples dont name their elements, so we have to use the index which is not clear
//     // we can't explicitly differentiate between which is the height and which is the width
// }

// REFACTORED AGAIN

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!(
//         "The area of the rectange is {} square pixels.",
//         // below we pass a reference to the struct
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// REFACTORED AGAIN AGAIN lol

// #[derive(Debug)]
// // above will print out debugging information
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 3;
//     let rect1 = Rectangle {
//         // dbg is a macro takes ownership of an expression, prints the file and line num of where it was called along w/ the resulting value of that expression
//         // dbg returns ownership of the value 
//         // dbg prints to the standrd error console stream (stderr)
//         // whereas println! prints to the standard output stream (stdout)
//         width: dbg!(30*scale),
//         height: dbg!(50*scale),
//     };
//     println!(
//         "rect1 is {:?}",
//         rect1
//     );
//     println!(
//         "The area of the rectange is {} square pixels.",
//         // below we pass a reference to the struct
//         area(&rect1)
//     );
// }

// REFACTORED AGAIN AGAIN AGAIN!!!!!

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of rectangle is {} square pixels.",
        rect1.area()
    );
}

