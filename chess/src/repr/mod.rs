pub mod bitboard;
pub mod pieces;


pub use pieces::{Pieces, Piece};
pub use bitboard::{Bitboard, Square, Direction};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Normal(Square, Square, Piece),
    EnPassant(Square, Square),
    Promotion(Square, Square, Piece),
    KingsideCastle,
    QueensideCastle
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Normal(src, dst, _) => write!(f, "{}{}", src,dst),
            Move::EnPassant(src, dst) => write!(f, "{}{}", src,dst),
            Move::Promotion(src,dst,to) => match to {
                Piece::Queen => write!(f, "{}{}q", src,dst),
                Piece::Rook => write!(f, "{}{}r", src,dst),
                Piece::Bishop => write!(f, "{}{}b", src,dst),
                Piece::Knight => write!(f, "{}{}n", src,dst),
                _ => panic!("Can't promote to that piece"),
            }
            Move::KingsideCastle => write!(f, "O-O"),
            Move::QueensideCastle => write!(f, "O-O-O"),
        }
    }
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn other(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Colored<T>(pub T,pub T);


impl<T> Index<Color> for Colored<T>{
    type Output = T;
    fn index(&self, color: Color) -> &T {
        match color {
            Color::White => &self.0,
            Color::Black => &self.1
        }
    }
}

impl<T> IndexMut<Color> for Colored<T> {
    fn index_mut(&mut self, color: Color) -> &mut T {
        match color {
            Color::White => &mut self.0,
            Color::Black => &mut self.1
        }
    }
}



#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CastlingRights {
    pub kingside: bool,
    pub queenside: bool,
}

impl CastlingRights {
    pub fn new(kingside: bool, queenside: bool) -> CastlingRights {
        CastlingRights { kingside, queenside }
    }
}


