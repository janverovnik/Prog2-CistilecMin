mod strukture;
mod generator_polja;
mod display;
mod gameplay;
use std::io;

// use crate generator_polja::{safe_new};
use crate::strukture::{Tile,Mreza};
use crate::gameplay;


fn main() {
    println!("Select seed");

    let mut seed = String::new();
    let mut poteza = String::new();

    io::stdin()
        .read_line(&mut seed)
        .expect("Failed to read line");

    let seed: u64 = match seed.trim().parse() {
        Ok(num) => num,
        Err(_) => 42,
    };
    
    let mut mreza = Mreza::safe_new((16,16),40,seed);

    loop {
        print!("{}", mreza);
        println!("Naredi potezo!");

        io::stdin()
        .read_line(&mut poteza)
        .expect("Failed to read line");

        let pot: Option<(char, u16, u16)> = match poteza.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };
    match pot {
        | None => continue,
        | Some(('U', x, y)) | Some (('u', x, y)) => mreza.change_flag((x,y)),
        | Some(('F', x, y)) | Some (('f', x, y)) => mreza.uncover_tile((x,y)),
        | _ => continue
    }    
    }

}
