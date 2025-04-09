use std::collections::HashMap;


pub enum Mark {
    Flagged,
    NotFlagged,
}

pub enum Status {
    Open,
    Closed(Mark),
}

pub enum Vsebina {
    Stevilo(u8),
    Mina,
}

pub struct Tile {
    vsebina: Vsebina,
    status: Status,
    mesto: (u16, u16)
}

pub struct Mreza {
    velikost: (u16, u16),
    tiles: HashMap<(u16, u16), Tile>,
}
// let field = HashMap<(u32,u32), Tile>


impl Tile {
    pub fn vsebina(&self) -> &Vsebina {
        &self.vsebina
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}

impl Mreza {
    pub fn tile(&self, i: u16, j: u16) -> Option<&Tile> {
        self.tiles.get(&(i, j))
    }
    pub fn add_tile(&mut self, tile:Tile ,i:u16 ,j:u16) -> Option<Tile>  {
        self.tiles.insert((i,j),tile)
    }
}
    

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn poizkus() {
        let test_mreza = Mreza {
            velikost: (123,456),
            tiles: HashMap::new()
        };
    }
}


    // fn preverjaj(&self) -> bool {
    //     match self.vsebina {

    //     }
    // }