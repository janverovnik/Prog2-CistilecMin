use std::collections::HashTable;


enum Status {
    Open,
    Closed,
}

enum Vsebina {
    Stevilo(u8),
    Mina,
}

struct Tile {
    vsebina : Vsebina,
    status : Status,
}

// let field = HashMap<(u32,u32), Tile>


impl Tile {
    pub fn vsebina(&self) -> Vsebina {
        self.vsebina
    }

    pub fn status(&self) -> Status {
        self.status
    }

    

    // fn preverjaj(&self) -> bool {
    //     match self.vsebina {

    //     }
    // }
}