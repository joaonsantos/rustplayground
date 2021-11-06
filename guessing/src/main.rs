use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let value = rand::thread_rng().gen_range(1..10);
    println!("Guess the number!");

    loop {
        println!("Please input a number:");

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Could not parse input");

        println!("You guessed: {}", guess);

        let guess: u32 = guess.trim().parse().unwrap();
        match guess.cmp(&value) {
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Less => println!("Too small!"),
        };
    }
}
