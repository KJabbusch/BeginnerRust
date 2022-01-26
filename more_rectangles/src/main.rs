fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // we use :: notation to access the associated function
    let sq1 = Rectangle::square(2);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold sq1? {}", rect1.can_hold(&sq1));

}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // we can use multiple impl blocks, but it is not required
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
    // this is called an associated function, it is not a method; does not contain a self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

