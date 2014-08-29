use std::io;
use std::rand;

fn main() {
    println!("Guess the magic number!");
    let magic_number = ( rand::random::<uint>() % 100u ) + 1u;
    loop {
        let input = io::stdin().read_line().ok().expect("That was bad input");
        let input_optoin_uint: Option<uint> = from_str(input.as_slice().trim());
        let input_uint = match input_optoin_uint {
            Some(num) => num,
            None => {
                println!("Please input a number");
                continue;
            }
        };
        println!("Guess = {}", input_uint);
        match compaire(input_uint, magic_number) {
            Less => println!("Guess higher."),
            Greater => println!("Guess lower."),
            Equal => { 
                println!("You win!"); 
                return
            },
        }
    }
}

fn compaire(input: uint, test: uint) -> Ordering {
    if input < test {
        Less
    }
    else if input > test {
        Greater
    }
    else {
        Equal
    }
}
