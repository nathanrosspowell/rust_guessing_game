extern crate rust_guessing_game;

use std::old_io as io;
use std::rand;
use std::cmp::Ordering;

use rust_guessing_game::my_funcs;

#[cfg(not(test))]
fn main() {
    println!("Guess the magic number!");
    let magic_number = ( rand::random::<uint>() % 100u ) + 1u;
    loop {
        let input = io::stdin().read_line().ok().expect("That was bad input");
        let input_optoin_uint: Option<uint> = input.parse();
        let input_uint = match input_optoin_uint {
            Some(num) => num,
            None      => {
                println!("Please input a number");
                continue;
            }
        };
        match my_funcs::compare(input_uint, magic_number) {
            Ordering::Less    => println!("Guess higher."),
            Ordering::Greater => println!("Guess lower."),
            Ordering::Equal   => { 
                println!("You win!"); 
                return
            },
        }
    }
}
