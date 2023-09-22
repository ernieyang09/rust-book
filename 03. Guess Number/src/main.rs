use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number");

    let secret = rand::thread_rng().gen_range(1..=100);

    

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("input fail");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessing number: {guess}");

        match guess.cmp(&secret) {
            Ordering::Greater => println!("guess lower"),
            Ordering::Less => println!("guess higher"),
            Ordering::Equal  => {
                println!("you got it");
                break;
            },
        }
    }

}
