extern crate rust_guessing_game;

use std::old_io as io;
use std::rand;
use std::cmp::Ordering;

use rust_guessing_game::my_funcs;

#[cfg(not(test))]
fn main() {
    println!("Guess the magic number!");
    let magic_number = ( rand::random::<usize>() % 100 ) + 1;
    loop {
        let input = io::stdin().read_line().ok().expect("That was bad input");
        let input_optoin_usize: Option<usize> = input.parse();
        let input_usize = match input_optoin_usize {
            Some(num) => num,
            None      => {
                println!("Please input a number");
                continue;
            }
        };
        match my_funcs::compare(input_usize, magic_number) {
            Ordering::Less    => println!("Guess higher."),
            Ordering::Greater => println!("Guess lower."),
            Ordering::Equal   => { 
                println!("You win!"); 
                return
            },
        }
    }
}
