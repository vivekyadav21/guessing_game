use std::{io, cmp::Ordering};
use rand::prelude::{*, self};


fn main() {
    println!("Guess the number");
    let secret_number= prelude::thread_rng().gen_range(1..=100);
    //println!("secret number is : {secret_number}");
    loop{
        println!("Please input your guess");
        let mut guess=String::new();
        
        io::stdin().read_line(&mut guess).expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>continue,
        };
        println!("You guessed:{guess}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won");
                break;                        
        }
        };
    }

}
