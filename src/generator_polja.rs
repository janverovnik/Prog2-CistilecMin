// use std::collections::HashMap;

use rand::Rng;

use crate::strukture::{Tile,Mreza};


impl Mreza {
    pub fn new(velikost : (u16,u16), st_min: u16) -> Mreza {
        let mut mreza = Mreza::prazna(velikost);
        let mut zaporedje = random_array(velikost.0 * velikost.1, st_min).into_iter();
        let mut naslednji : bool;
        for i in 0..velikost.0 {
            for j in 0..velikost.1 {
                naslednji = zaporedje.next().is_some();
                if naslednji {
                    mreza.add_tile(Tile::new_bomb(), (i,j));
                }
            }
        };
        for i in 0..velikost.0 {
            for j in 0..velikost.1 {
                if mreza.je_prazno((i,j)) {
                    mreza.add_tile(Tile::new_number(mreza.pripisi_stevilo((i, j))),(i,j));
                }
            }
        };
        mreza
    }
}

fn random_array(mut st_vseh:u16,mut st_min:u16) -> Vec<bool> {
    let mut rng = rand::rng();
    let mut nakljucno = vec![];
    loop {
        if rng.random_ratio(st_min.into(),st_vseh.into()) {
            nakljucno.push(true);
            st_min -= 1;
            st_vseh -= 1;
        } else {
            nakljucno.push(false);
            st_vseh -= 1
        }
        if st_vseh == 0 {
            break;
        }
    }
    nakljucno
}

