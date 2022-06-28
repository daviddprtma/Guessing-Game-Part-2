
// use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::iter::repeat_with;

fn main() {
    println!("Guess the number!");

    let s:String = repeat_with(fastrand::lowercase).take(10).collect();
    println!("The random alphabet is: {}",s);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: String = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&s) {
            Ordering::Less => println!("Letter is too small!"),
            Ordering::Greater => println!("Letter is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
