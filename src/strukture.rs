use std::collections::HashMap;

pub enum Mark {
    Flagged,
    NotFlagged,
    Safe,
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
    pub vsebina: Vsebina,
    pub status: Status,
}

pub struct Mreza {
    pub velikost: (u16, u16),
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
        }
    }
    
    pub fn new_number(stevilo: u8) -> Tile {
        Tile {
            vsebina: Vsebina::Stevilo(stevilo),
            status: Status::Closed(Mark::NotFlagged),
        }
    }

    pub fn new_safe() -> Tile {
        Tile { 
            vsebina: Vsebina::Stevilo(0), 
            status: Status::Closed(Mark::Safe)
        }
    }

    pub fn uncover(&mut self) -> () {
        self.status = Status::Open  
    }

    pub fn change_flag(&mut self) -> () {
        self.status = match self.status {
            Status::Open =>  Status::Open,
            Status::Closed(Mark::Flagged) => Status::Closed(Mark::NotFlagged),
            Status::Closed(Mark::NotFlagged) => Status::Closed(Mark::Flagged),
            Status::Closed(Mark::Safe) => panic!("Cannot change flags before game start!")
        }
    }
}

impl Mreza {
    pub fn tile(&self, mesto:(u16, u16)) -> Option<&Tile> {
        self.tiles.get(&mesto)
    }
    
    pub fn add_tile(&mut self, tile:Tile ,mesto:(u16, u16)) -> Option<Tile>  {
        self.tiles.insert(mesto, tile)
    }

    pub fn mines(&self) -> Vec<(u16, u16)> {
        let mut mine_vec = vec![];
        for ((i, j), tile) in self.tiles.iter() {
            if tile.vsebina == Vsebina::Mina {
                mine_vec.push((*i, *j));
            }
        }
        return mine_vec;
    }

    pub fn sosedje(&self, mesto:(u16, u16)) -> Vec<(u16, u16)> {
        let (i,j) = mesto;
        let mut mozni = match (i,j) {
            (0,0) => vec![(0,1),(1,0),(1,1)],
            (0,j) => vec![(0, j - 1), (1, j - 1), (1, j), (0, j + 1), (1, j + 1)],
            (i,0) => vec![(i - 1, 0), (i + 1, 0),(i - 1, 1), (i, 1), (i + 1, 1)],
            (_,_) => vec![(i - 1, j - 1), (i, j - 1), (i + 1, j - 1),
            (i - 1, j), (i + 1, j),
        (i - 1, j + 1), (i, j + 1), (i + 1, j + 1)],
        };
    let keys: Vec<&(u16, u16)>  = self.tiles.keys().collect();
    mozni.retain(|n| keys.contains(&n));
    return mozni;
    }

    pub fn pripisi_stevilo(&self, mesto:(u16, u16)) -> u8 {
        let mut stevec: u8 = 0;
        for sosed in &self.sosedje(mesto) {
            match self.tile(*sosed) {
                None => stevec += 0,
                Some(tile) => 
                    if (*tile).vsebina == Vsebina::Mina {
                        stevec += 1
                }
            }
        }
        stevec
    }
    
    pub fn sosednje_zastavice(&self, mesto:(u16, u16)) -> u8 {
        let mut stevec: u8 = 0;
        for sosed in &self.sosedje(mesto) {
            match self.tile(*sosed) {
                None => stevec += 0,
                Some(tile) => 
                    match tile.status {
                        Status::Closed(Mark::Flagged) => stevec += 1,
                        _ => ()
                    }
                }
            }
        stevec
        }
        
    
    
    pub fn prazna(velikost: (u16, u16)) -> Mreza {
        Mreza {
            velikost: velikost,
            tiles: HashMap::new(),
        }
    }

    pub fn je_prazno(&self,mesto:(u16,u16)) -> bool {
        !self.tiles.contains_key(&mesto)
    }

     fn tile_mut(&mut self, mesto:(u16, u16)) -> Option<&mut Tile> {
        self.tiles.get_mut(&mesto)
    }   
    
    pub fn apply_on_tile<F>(&mut self,mut f: F, mesto:(u16, u16)) -> ()
    where 
        F : FnMut(&mut Tile) ->  ()  {
        match self.tile_mut(mesto) {
            None => (),
            Some(tile) => f(tile),
        }
    }

}