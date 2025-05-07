use crate::strukture::{Mark,Status,Tile,Mreza, Vsebina};


impl Mreza {
    pub fn uncover_tile(&mut self, mesto: (u16,u16)) {
        let f = |tile: &mut Tile| {tile.uncover()};
        self.apply_on_tile(f , mesto)
    }

    pub fn change_flag(&mut self, mesto: (u16,u16)) {
        let f = |tile: &mut Tile| {tile.change_flag()};
        self.apply_on_tile(f , mesto)
    } 

    pub fn empty_uncover(&mut self, mesto: (u16,u16)) { // uncover za prazne tile-e, se odprejo vsi okoli itd.
        let f = |tile: &mut Tile| {tile.uncover()};
        for sosed in self.sosedje(mesto) {
            self.apply_on_tile(f, sosed);
        }
    }

    pub fn mine_uncover(&mut self, mesto: (u16,u16)) {
        // TODO
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