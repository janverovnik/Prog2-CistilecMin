use std::collections::HashMap;

#[derive(PartialEq)]
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
    pub velikost: (usize, usize),
    // tiles: HashMap<(u16, u16), Tile>,
    tiles: Vec<Vec<Option<Tile>>>
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

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile { vsebina: Vsebina::Mina, status: Status::Open }
    }
}
// Clone nima nobene pomembne naloge, mora le biti definirano

impl Mreza {
    pub fn tile(&self, (i,j):(usize, usize)) -> Option<&Tile> {
        self.tiles[i][j].as_ref()
    }
    
    pub fn add_tile(&mut self, tile:Tile ,(i,j):(usize, usize)) -> Option<Tile>  {
        self.tiles[i][j] = Some(tile);
        None
    }

    pub fn mines(&self) -> Vec<(usize, usize)> {
        let mut mine_vec = vec![];
        let (m,n) = self.velikost;
        for i in 0..m {
            for j in 0..n {
                match self.tile((i,j)) {
                    Some(tile) => 
                    if (*tile).vsebina == Vsebina::Mina {mine_vec.push((i, j))},
                    _ => (),
                }
            }
        }
        return mine_vec
    }
    

    pub fn sosedje(&self, mesto:(usize, usize)) -> Vec<(usize, usize)> {
        let (m_plus,n_plus) = self.velikost;
        let (m,n) = (m_plus-1,n_plus-1);
        let (i,j) = mesto;
        let mozni = if (i,j) == (m,0) {
            vec![(m-1,0),(m-1,1),(m,1)]
        } else if (i,j) == (0,n) {
            vec![(0,n-1), (1,n-1), (1,n)]
        } else if (i,j) == (m,n) {
            vec![(m-1,n),(m-1,n-1),(m,n-1)]
        } else if i == m {
            vec![(i,j-1), (i-1,j-1), (i-1,j), (i-1,j+1), (i,j+1)]
        } else if j == n {
            vec![(i-1,j), (i-1,j-1), (i,j-1), (i+1,j-1), (i+1,j)]
        } else {match (i,j) {
            (0,0) => vec![(0,1),(1,0),(1,1)],
            (0,j) => vec![(0, j - 1), (1, j - 1), (1, j), (0, j + 1), (1, j + 1)],
            (i,0) => vec![(i - 1, 0), (i + 1, 0),(i - 1, 1), (i, 1), (i + 1, 1)],
            (_,_) => vec![(i - 1, j - 1), (i, j - 1), (i + 1, j - 1),
            (i - 1, j), (i + 1, j),
        (i - 1, j + 1), (i, j + 1), (i + 1, j + 1)],
        }
        };
    return mozni
    }
    // Ocaml match VELIKO BOLJ superioren od rust matcha

    pub fn pripisi_stevilo(&self, mesto:(usize, usize)) -> u8 {
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
    
    pub fn sosednje_zastavice(&self, mesto:(usize, usize)) -> u8 {
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
        

    pub fn prazna(velikost: (usize, usize)) -> Mreza {
        Mreza {
            velikost: velikost,
            tiles:  vec![vec![None;velikost.1];velikost.0],
        }
    }

    pub fn je_prazno(&self,mesto:(usize,usize)) -> bool {
        match self.tile(mesto) {
            None => true,
            Some(_) => false,
        }
    }

     fn tile_mut(&mut self, mesto:(usize, usize)) -> Option<&mut Tile> {
        self.tiles[mesto.0][mesto.1].as_mut()
    }   
    
    pub fn apply_on_tile<F>(&mut self,mut f: F, mesto:(usize, usize)) -> ()
    where 
        F : FnMut(&mut Tile) ->  ()  {
        match self.tile_mut(mesto) {
            None => (),
            Some(tile) => f(tile),
        }
    }

}