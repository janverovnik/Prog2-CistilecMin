use std::collections::HashMap;

use rand::Rng;

use crate::strukture::{Tile,Mreza};


impl Mreza {
    pub fn new(velikost : (u16,u16), st_min: u16) -> Mreza {
        let mut mreza = Mreza::prazna(velikost);
        let mut zaporedje = random_array(velikost.0 * velikost.1, st_min).iter();
        let mut naslednji : bool
        for i in 0..velikost.0 {
            for j in 0..velikost.1 {
                naslednji = zaporedje.next()
                if naslednji {
                    mreza.add_tile(Tile::new_bomb(i,j), (i,j));
                }
            }
        };
        
    }
}

fn random_array(st_vseh:u16,st_min:u16) -> Vec<bool> {
    let mut rng = rand::rng();
    let nakljucno = vec![];
    loop {
        if rng.random_ratio(st_min,st_vseh) {
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