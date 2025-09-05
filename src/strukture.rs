
#[derive(PartialEq,Clone,Copy,Debug,Eq)]
pub enum Mark {
    Flagged,
    NotFlagged,
    Safe,
}
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Status {
    Open,
    Closed(Mark),
}

#[derive(PartialEq,Clone,Copy,Debug,Eq)]
pub enum Vsebina {
    Stevilo(u8),
    Mina,
}

use crate::{Resource,Component};
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    pub vsebina: Vsebina,
    pub status: Status,
}

    

pub struct Mreza {
    pub velikost: (usize, usize),
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
}

impl Mreza {
    pub fn tile(&self, (i,j):(usize, usize)) -> Option<&Tile> {
        self.tiles[i][j].as_ref()
    }
    
    pub fn add_tile(&mut self, tile:Tile ,(i,j):(usize, usize)) -> Option<Tile>  {
        self.tiles[i][j] = Some(tile);
        None
    }

    pub fn sosedje(&self, mesto:(usize, usize)) -> Vec<(usize, usize)> { // TODO: POLEPŠAJ!!
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

}