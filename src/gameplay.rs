use crate::strukture::{Mark,Status,Tile,Mreza};

impl Mreza {
    pub fn uncover_tile(&mut self, mesto: (u16,u16)) -> () {
        let f = |tile: &mut Tile| {tile.uncover()};
        self.apply_on_tile(f , mesto)
    }

    pub fn change_flag(&mut self, mesto: (u16,u16)) -> () {
        let f = |tile: &mut Tile| {tile.change_flag()};
        self.apply_on_tile(f , mesto)
    } 
}