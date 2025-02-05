use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guessing game");
    println!("enter number:");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number = {secret_number}");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("w");
                break;
            }
        }
    }
}
