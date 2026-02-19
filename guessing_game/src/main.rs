use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 0.10.0 rng().random_range(1..=100);

    println!("Guess the number!");

    loop {
        //while
        println!("Please input your guess.");

        let mut guess = String::new();
        // mut definisce una variabile mutabile (al contrario di const in c++), String::new() crea una nuova stringa vuota

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // stdin ritorna istanza di standard imput, passaggio per rifermimento, passa lindirizzo di memoria della variabile guess come in c,
        // expect e' un metodo dellenum result puo essere ok o err e va chiamato expect perche se no da warning

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            //switch expression la match accetta le enum
            Ok(num) => num,
            Err(_) => continue, // continue skippa alla prossima iterazione di loop
        };

        println!("You guess: {guess}");

        match guess.cmp(&secret_number) {
            // ordering in questo caso e' un enum e il compare restituisce come risultato un ordering che puo essere uno di quei tre.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {secret_number}");
}
