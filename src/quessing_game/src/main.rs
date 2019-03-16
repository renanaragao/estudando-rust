extern crate rand;
#[macro_use()]
extern crate pipeline;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let result = pipe!(
        //     guess,
        //     => [trim()]
        //     => [parse()]
        //     => [cmp(&secret_number)]
        // );

        println!("You guessed: {}", guess);
        println!("Secrect number: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
