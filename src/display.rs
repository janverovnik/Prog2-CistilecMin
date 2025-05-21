// #![allow(non_snake_case)]

use std::fmt::{self, write, Display, Formatter};
use crate::strukture::{Mark, Mreza, Tile};
use crate::strukture::Vsebina::{Mina, Stevilo};
use crate::strukture::Status;

impl Display for Mreza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut niz : String = Default::default();
    let mut dodatek : String;
    let velikost = self.velikost;

    for i in 0..velikost.0 {

        for j in 0..velikost.1 {
            dodatek = match self.tile((i,j)) {
                None => String::from(" "),
                Some(tile) => match *tile.status() {
                    Status::Open =>  match *tile.vsebina() {
                            Stevilo(x) => x.to_string() + " ",
                            Mina => String::from("* "),
                    },
                    Status::Closed(Mark::Safe) => String::from("X "),
                    Status::Closed(Mark::NotFlagged) => String::from("□ "),
                    Status::Closed(Mark::Flagged) => String::from("F "),
            }
            };
            niz.push_str(&dodatek);

        }
        niz.push_str(&"\n".to_string());
    
    }
    write!(f, "{}", niz)
    }
}

#[cfg(test)]
mod tests {
    use crate::strukture::Mreza;

    #[test]
    fn printaj() -> () {
        let mreza = Mreza::new((16,16),40,42);
        print!("{}", mreza);
    }
    #[test]
    fn printaj_safe() -> () {
        let mreza = Mreza::safe_new((16,16),40,42);
        print!("{}", mreza);
    }
    // Implementacija Display verjetno ne bo pomembna pri končnem izdelku, vendar je priročna za ugotavljanje in odpravljanje težav.
}