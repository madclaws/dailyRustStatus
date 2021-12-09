#[derive(Debug)]
pub struct ChessPosition{
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen{
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition{rank, file})
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank {
            true
        } else if self.position.file == other.position.file {
            true
        } else if i32::abs(self.position.rank - other.position.rank)
            == i32::abs(self.position.file - other.position.file) {
                true
        } else {
            false
        }
    }
}
