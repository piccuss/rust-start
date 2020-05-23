use std::io;
use std::cmp::{Ordering, Ord};
use rand::Rng;

pub fn guess() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input!");

        let mut result = String::new();

        io::stdin()
            .read_line(&mut result)
            .expect("error inout!");

        let result: u32 = match result.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your input: {}", result);

        match result.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}