use crate::strukture::{Tile, Mreza, Vsebina};

impl Mreza {
    pub fn uncover_tile(&mut self, mesto: (u16,u16), forbidden: &mut Vec<(u16,u16)>) {
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

    pub fn change_flag(&mut self, mesto: (u16,u16)) {
        let f = |tile: &mut Tile| {tile.change_flag()};
        self.apply_on_tile(f, mesto)
    } 

    pub fn sure_uncover(&mut self, mesto: (u16,u16)) { // če imaš dva zaprta sosednja tile-a flaggana za mino lahko na tile dvakrat pritisneš da se odprejo ostali (at your own peril)W
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