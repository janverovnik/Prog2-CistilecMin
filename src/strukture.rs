use std::collections::HashMap;


pub enum Mark {
    Flagged,
    NotFlagged,
}

pub enum Status {
    Open,
    Closed(Mark),
}

#[derive(PartialEq)]
pub enum Vsebina {
    Stevilo(u8),
    Mina,
}

pub struct Tile {
    vsebina: Vsebina,
    status: Status,
    // mesto: (u16, u16)
}

pub struct Mreza {
    velikost: (u16, u16),
    tiles: HashMap<(u16, u16), Tile>,
}



impl Tile {
    pub fn vsebina(&self) -> &Vsebina {
        &self.vsebina
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn new_bomb() -> Tile {
        Tile {
            vsebina: Vsebina::Mina,
            status: Status::Closed(Mark::NotFlagged),
            // mesto: mesto,
        }
    }
    
    pub fn new_number(stevilo: u8) -> Tile {
        Tile {
            vsebina: Vsebina::Stevilo(stevilo),
            status: Status::Closed(Mark::NotFlagged),
            // mesto: mesto,
        }
    }
}

use crate::strukture::Vsebina::Mina;

impl Mreza {
    pub fn tile(&self, mesto:(u16,u16)) -> Option<&Tile> {
        self.tiles.get(&mesto)
    }
    pub fn add_tile(&mut self, tile:Tile ,mesto:(u16,u16)) -> Option<Tile>  {
        self.tiles.insert(mesto,tile)
    }

    // pub fn add_tile(&mut self, tile: Tile, i: u16, j: u16) -> Option<Tile> {
        //     self.tiles.insert((i, j), tile)
        pub fn mines(&self) -> Vec<(u16, u16)> {
            let mut mine_vec = vec![];
            for ((i, j), tile) in self.tiles.iter() {
                if tile.vsebina == Mina {
                    mine_vec.push((*i, *j));
                }
            }
            return mine_vec;
        }
        pub fn sosedje(&self, mesto:(u16,u16)) -> Vec<(u16, u16)> {
            let (i,j) = mesto;
            let mut mozni = vec![(i - 1, j - 1), (i, j - 1), (i + 1, j - 1),
            (i - 1, j), (i + 1, j),
        (i - 1, j + 1), (i, j + 1), (i + 1, j + 1)];
        let keys: Vec<&(u16, u16)>  = self.tiles.keys().collect();
        mozni.retain(|n| keys.contains(&n));
        return mozni;
    }
    pub fn pripisi_stevilo(&self, mesto:(u16,u16)) -> u8 {
        let (i,j) = mesto;
        2 //TODO
    }
    
    pub fn prazna(velikost: (u16,u16)) -> Mreza {
        Mreza {
            velikost: velikost,
            tiles: HashMap::new(),
        }
    }

    pub fn je_prazno(&self,mesto:(u16,u16)) -> bool {
        !self.tiles.contains_key(&mesto)
    }
}
    
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn poizkus() {
        let test_mreza = Mreza {
            velikost: (123, 456),
            tiles: HashMap::new()
        };
       
    }
}


    // fn preverjaj(&self) -> bool {
    //     match self.vsebina {

    //     }
    // }