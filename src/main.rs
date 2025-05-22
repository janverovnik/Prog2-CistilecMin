mod strukture;
mod generator_polja;
mod display;
mod gameplay;
use std::io;

use crate::strukture::{Mreza, Vsebina};



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
    
    let mut mreza = Mreza::safe_new((16,16),40,seed);

    loop {
        print!("{}", mreza);
        println!("Naredi potezo!");
        
        let mut poteza = String::new();

        io::stdin()
        .read_line(&mut poteza)
        .expect("Failed to read line");

        let mut iter = poteza.split_whitespace();
        let crka_opt = iter.next();
        let x_opt = iter.next();
        let y_opt = iter.next();

        let (crka,x,y) = match (crka_opt,x_opt,y_opt) {
            (Some(crka),Some(x),Some(y)) => (crka.parse(),x.parse(),y.parse()),
            _ => (Ok('X'),Ok(42),Ok(42)),
        };

        let pot : Option<(char,u16,u16)> = match (crka,x,y) {
            (Ok(crka),Ok(x),Ok(y)) => Some((crka,x,y)),
            _ => None,
        };
        
        match pot {
            | None => continue,
            | Some(('U', x, y)) | Some (('u', x, y)) => 
            {mreza.uncover_tile((x,y));
                match mreza.tile((x, y)) {
                None => (),
                Some(tile) => if *tile.vsebina() == Vsebina::Mina{ print!("{}\n{}\n", mreza, "KABOOM!"); break}}
            }
            | Some(('F', x, y)) | Some (('f', x, y)) => mreza.change_flag((x,y)),
            | _ => continue
        }    
    }

}
