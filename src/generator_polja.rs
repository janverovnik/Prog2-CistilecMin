use crate::strukture::{Tile,Mreza};

fn razlika(m:u16,n:u16) -> u16 {
    if m > n {
        m - n
    } else {
        n - m
    }
}

impl Mreza {
    pub fn new(velikost : (u16,u16), st_min: u16, seed : u64) -> Mreza {
        let mut mreza = Mreza::prazna(velikost);
        let mut zaporedje = random_array_homemade(velikost.0 * velikost.1, st_min, seed).into_iter();
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

    pub fn safe_new(velikost: (u16,u16),st_min: u16, seed : u64) -> Mreza {
        let (m, n) = (velikost.0 - 1, velikost.1 - 1);
        let safe_space = rand_safe(seed, &velikost);
        let (s0,s1) = safe_space;
        let mut mreza = Mreza::prazna(velikost);
        let varne = if safe_space == (0,0) || safe_space == (m, 0) || safe_space == (0, n) || safe_space == (m, n) {
        // Ocaml match je superior od rust matcha
            4
        } else if s0 == 0 || s0 == m || s1 == 0 || s1 == n {
            6
        } else {
            9
        };
        let mut zaporedje =  random_array_homemade(velikost.0 * velikost.1 - varne, st_min,seed).into_iter();
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
        mreza.add_tile(Tile::new_safe(0),safe_space);
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

fn random_array_homemade(st_vseh:u16,st_min:u16,seed: u64) -> Vec<bool> {
    let mut vsi = st_vseh as u64;
    let mut mine = st_min as u64;
    let a: u64 = 674267;
    let b: u64 = 101010;
    let m = 123456;
    let mut x: u64 = seed;
    let mut nakljucno = vec![];

    loop {
        for _ in 0..97 {
            x = (a*x + b) % m;
        };
        if x % vsi < mine {
            nakljucno.push(true);
            mine -= 1;
            vsi -= 1;
        } else {
            nakljucno.push(false);
            vsi -= 1
        }
        if vsi == 0 {
            break;
        }
    }
    nakljucno
}

fn rand_safe(seed:u64, &velikost: &(u16,u16)) -> (u16,u16) {
    let n = velikost.0 as u64;
    let m = velikost.1 as u64;

    let a: u64 = 674267;
    let b: u64 = 101010;
    let m = 123456;
    let mut y: u64 = seed;

     
    for _ in 0..97 {
        y = (a*y + b) % m;
    }
    let x : u64 = y;

    for _ in 0..97 {
        y = (a*y + b) % m;
    };
    ((x % n) as u16,(y % m) as u16)
}



#[cfg(test)]
mod tests {
    use super::random_array_homemade;

    #[test]
    fn permutacija() -> () {
        let binding = random_array_homemade(10, 3,6539);
        let mut vector_iter = binding.iter();
        let mut nasledniji : bool;
        for _ in 0..10 {
            nasledniji = *vector_iter.next().expect("Test ne deluje");
            print!("{}, ", nasledniji);
        };
    }
    // Ta test je bil narejen, ker je testna funkcija printaj() v display.rs izrisala samo mine. Napaka ni bila v funkciji random_array().
}