pub struct ChessPosition { x : isize, y : isize }

impl ChessPosition {
    pub fn new(x : isize, y : isize) -> Result<Self, &'static str> {
        if x < 0 || x > 7 || y < 0 || y > 7
            { Err("Invalid Position") }
        else
            { Ok(ChessPosition { x : x, y : y }) }
    }
}

pub struct Queen { pos : ChessPosition }

impl Queen {
    pub fn new(pos : ChessPosition) -> Self {
        Queen { pos : pos }
    }

    pub fn can_attack(&self, another : &Self) -> bool {
        let p1 = &self.pos;
        let p2 = &another.pos;
        p1.x == p2.x || p1.y == p2.y ||
            p1.x - p1.y == p2.x - p2.y ||
            p1.x + p1.y == p2.x + p2.y
    }
}
