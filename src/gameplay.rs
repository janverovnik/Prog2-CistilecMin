use crate::strukture::{Tile, Mreza, Vsebina};

impl Mreza {
    pub fn uncover_tile(&mut self, mesto: (usize,usize), forbidden: &mut Vec<(usize,usize)>) {
        let f = |tile: &mut Tile| {tile.uncover()};
        match self.tile(mesto) {
            None => (),
            Some(tile) => match *tile.vsebina(){
                Vsebina::Stevilo(x) => {self.apply_on_tile(f, mesto);
                                            (*forbidden).push(mesto);
                                            if x == 0 {for sosed in self.sosedje(mesto) {
                                                if !forbidden.contains(&sosed){
                                                (*forbidden).push(sosed);
                                                self.uncover_tile(sosed, forbidden);}
                                            }
                                        }
                },
                Vsebina::Mina => for mina in self.mines(){
                    self.apply_on_tile(f, mina)
                },

            }
        }
    }

    pub fn change_flag(&mut self, mesto: (usize,usize)) {
        let f = |tile: &mut Tile| {tile.change_flag()};
        self.apply_on_tile(f, mesto)
    } 

    pub fn sure_uncover(&mut self, mesto: (usize,usize)) { // če imaš dva zaprta sosednja tile-a flaggana za mino lahko na tile dvakrat pritisneš da se odprejo ostali (at your own peril)W
        match self.tile(mesto) {
            None => (),
            Some(&ref tile) => 
            match tile.vsebina() {
                &Vsebina::Mina => (),
                &Vsebina::Stevilo(x) => 
                if x == self.sosednje_zastavice(mesto)
                {let f = |tile: &mut Tile| {tile.uncover()};
                for sosed in self.sosedje(mesto) {
                    self.apply_on_tile(f, sosed);
                }}
                
            }
        }
    }
}


// mod tests {
//      use crate::strukture::{Mreza, Vsebina};
//     use std::io;;
//     use super::strukture;
//     use super::generator_polja;
//     use super::display;
//     use super::gameplay;
//     #[test]
//     fn igraj() -> () {

//     println!("Select seed");

//     let mut seed = String::new();
    
//     io::stdin()
//     .read_line(&mut seed)
//     .expect("Failed to read line");
    

//     let seed: u64 = match seed.trim().parse() {
//         Ok(num) => num,
//         Err(_) => 42,
//     };
    
//     let mut mreza = Mreza::safe_new((16,16),40,seed);

//     loop {
//         print!("\n{}", mreza);
//         println!("\nNaredi potezo!");
        
//         let mut poteza = String::new();

//         io::stdin()
//         .read_line(&mut poteza)
//         .expect("Failed to read line");

//         let mut iter = poteza.split_whitespace();
//         let crka_opt = iter.next();
//         let x_opt = iter.next();
//         let y_opt = iter.next();

//         let (crka,x,y) = match (crka_opt,x_opt,y_opt) {
//             (Some(crka),Some(x),Some(y)) => (crka.parse(),x.parse(),y.parse()),
//             _ => (Ok('X'),Ok(42),Ok(42)),
//         };

//         let pot : Option<(char,usize,usize)> = match (crka,x,y) {
//             (Ok(crka),Ok(x),Ok(y)) => Some((crka,x,y)),
//             _ => None,
//         };
        
//         match pot {
//             | None => continue,
//             | Some(('U', x, y)) | Some (('u', x, y)) => 
//             {mreza.uncover_tile((x,y), &mut vec![]);
//                 match mreza.tile((x, y)) {
//                 None => (),
//                 Some(tile) => if *tile.vsebina() == Vsebina::Mina{ print!("{}\n{}\n", mreza, "KABOOM!"); break}}
//             }
//             | Some(('F', x, y)) | Some (('f', x, y)) => mreza.change_flag((x,y)),
//             | _ => continue
//         }    
//     }
// }

// }