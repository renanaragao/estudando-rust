fn main() {
    struct_user();
    retangles();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn retangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Square: {:#?}", Rectangle::square(56));
}

fn struct_user() {
    let user = User {
        name: String::from("Renan"),
        user_name: String::from("raragao"),
        active: true,
    };

    println!("User A: {:#?}", user);

    let user = User {
        name: String::from("Rafael"),
        ..user
    };

    println!("User B: {:#?}", user);
}

#[derive(Debug)]
struct User {
    name: String,
    user_name: String,
    active: bool,
}
