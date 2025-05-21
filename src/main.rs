mod strukture;
mod generator_polja;
mod display;
mod gameplay;
use std::io;

// use crate generator_polja::{safe_new};
use crate::strukture::{Tile,Mreza};


fn main() {
    println!("Select seed");

    let mut seed = String::new();

    io::stdin()
        .read_line(&mut seed)
        .expect("Failed to read line");

    let seed: u64 = match seed.trim().parse() {
        Ok(num) => num,
        Err(_) => 42,
    };
    
    let mut mreza = Mreza::safe_new((16,16),40,(1,1),seed);
    let safe = generator_polja::ra

    loop {
        print!("{}", mreza);

    }



}
