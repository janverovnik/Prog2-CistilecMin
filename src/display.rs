// #![allow(non_snake_case)]

use std::fmt::{self, Display};
use crate::strukture::{Mark, Mreza};
use crate::strukture::Vsebina::{Mina, Stevilo};
use crate::strukture::Status;


fn png_select(tile: Tile) -> String{
    match tile.status() {
       Status::Open => match tile.vsebina {
            Mina => String::from("mina.png"),
            Stevilo(x) => format!("{x}.png")
       },
       Status::Closed(mark) => if *mark == Mark::Flagged {String::from("flag.png")} else if *mark == Mark::NotFlagged {String::from("top.png")} else {String::from("safe.png")}
    }
}

impl Display for Mreza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let velikost = self.velikost;
    let mut niz : String = Default::default();
    let mut dodatek : String;
    
    for j in 0..velikost.1 {
        
        for i in 0..velikost.0 {
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
        niz.push_str("\n");
    
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
    #[test]
    fn printaj_prazno() -> () {
        print!("{}", Mreza::prazna((8,8)))
    }
    
    // Implementacija Display verjetno ne bo pomembna pri končnem izdelku, vendar je priročna za ugotavljanje in odpravljanje težav.
}