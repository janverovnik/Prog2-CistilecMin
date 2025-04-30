use crate::strukture::{Mark,Status,Tile,Mreza};




// impl Tile {
//     fn uncover(mut self) -> () {
//         ()
//     }

//    const UNCOVER_C : FnMut<Tile> = |tile : Tile| -> Tile {

//    }

// }

impl Mreza {
    pub fn uncover_tile(&mut self, mesto: (u16,u16)) -> () {
        let f = |tile: &mut Tile| {
            match tile.status {
                Status::Closed(Mark::Flagged) => (),
                _ => tile.status = Status::Open,
            }
        };
        self.apply_on_tile(f , mesto)
    }

    pub fn change_flag(&mut self, mesto: (u16,u16)) -> () {
        let f = |tile: &mut Tile| {
            match tile.status {
                Status::Closed(Mark::Flagged) => tile.status = Status::Closed(Mark::NotFlagged),
                Status::Closed(Mark::NotFlagged) => tile.status = Status::Closed(Mark::Flagged),
                _ => (),
            }
        };
        self.apply_on_tile(f , mesto)
    }
   
}