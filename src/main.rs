extern crate rust_guessing_game;
extern crate rand;

use std::io as io;
use std::cmp::Ordering;

use rust_guessing_game::my_funcs;

#[cfg(not(test))]
fn main() {
    println!("Guess the magic number!");
    let magic_number = ( rand::random::<u32>() % 100 ) + 1;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("That was bad input");
        let input_num: Result<u32, _> = input.trim().parse();
        let num = match input_num {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        match my_funcs::compare(num, magic_number) {
            Ordering::Less    => println!("Guess higher."),
            Ordering::Greater => println!("Guess lower."),
            Ordering::Equal   => { 
                println!("You win!"); 
                return
            },
        }
    }
}
