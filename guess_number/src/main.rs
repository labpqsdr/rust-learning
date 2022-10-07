use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("game begin...");
    let secret_num = rand::thread_rng().gen_range(0..100);
    println!("guess a number:");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("wrong!");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your number is {}", guess);


        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}