fn main() {
    let user = User {
        name: String::from("Renan"),
        user_name: String::from("raragao"),
        active: true
    };

    println!("User A: {:?}", user);

    let user = User {
        name: String::from("Rafael"),
        ..user
    };

    println!("User A: {:?}", user);
}

#[derive(Debug)]
struct User {
    name: String,
    user_name: String,
    active: bool
}