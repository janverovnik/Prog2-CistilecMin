use rand::Rng;
use crate::strukture::{Tile,Mreza};

fn razlika(m:u16,n:u16) -> u16 {
    if m > n {
        m - n
    } else {
        n - m
    }
}

impl Mreza {
    pub fn new(velikost : (u16,u16), st_min: u16) -> Mreza {
        let mut mreza = Mreza::prazna(velikost);
        let mut zaporedje = random_array(velikost.0 * velikost.1, st_min).into_iter();
        let mut naslednji : bool;
        for i in 0..velikost.0 {
            for j in 0..velikost.1 {
                naslednji = match zaporedje.next() {
                    Some(t) => t,
                    None => true,
                };
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

    pub fn safe_new(velikost: (u16,u16),st_min: u16, safe_space: (u16,u16)) -> Mreza {
        let (m, n) = (velikost.0 - 1, velikost.1 - 1);
        let (s0,s1) = safe_space;
        let mut mreza = Mreza::prazna(velikost);
        // Ocaml match je superior od rust matcha
        let varne = if safe_space == (0,0) || safe_space == (m, 0) || safe_space == (0, n) || safe_space == (m, n) {
            4
        } else if s0 == 0 || s0 == m || s1 == 0 || s1 == n {
            6
        } else {
            9
        };
        let mut zaporedje =  random_array(velikost.0 * velikost.1 - varne, st_min).into_iter();
        let mut naslednji : bool;
        for i in 0..velikost.0 {
            for j in 0..velikost.1 {
               if razlika(i,s0) > 1 || razlika(j,s1) > 1 {
                naslednji = match zaporedje.next() {
                    Some(t) => t,
                    None => true,
                };
                        if naslednji {
                            mreza.add_tile(Tile::new_bomb(), (i,j));
                        };
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

#[cfg(test)]
mod tests {
    use super::random_array;

    #[test]
    fn permutacija() -> () {
        let binding = random_array(10, 3);
        let mut vector_iter = binding.iter();
        let mut nasledniji : bool;
        for _ in 0..10 {
            nasledniji = *vector_iter.next().expect("Test ne deluje");
            print!("{}, ", nasledniji);
        };
    }
    // Ta test je bil narejen, ker je testna funkcija printaj() v display.rs izrisala samo mine. Napaka ni bila v funkciji random_array().
}