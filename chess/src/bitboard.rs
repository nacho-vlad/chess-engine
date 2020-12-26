use std::iter::Iterator;
use std::ops::Add;
use std::str::FromStr;
use std::fmt::Display;
use strum_macros::EnumIter;
use crate::ChessError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd, EnumIter)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}



impl Square {
    pub fn to_bitboard(self) -> Bitboard {
        let sq = self as u8;
        Bitboard(1 << (8 * (sq/8) +  (sq%8)))
    }

    pub fn rank(self) -> u8 {
        (self as u8)/8
    }

    pub fn file(self) -> u8 {
        (self as u8)%8
    }

    pub fn ray(self, direction: Direction) -> Ray {
        Ray { square: self as i8, direction}
    }
}


impl FromStr for Square {
    type Err = ChessError;

    fn from_str(string: &str) -> Result<Square, ChessError> {
        if string.len() != 2 {
            return Err(ChessError::ParseSquare(string.to_owned()));
        }
        
        let mut chars = string.chars();
        let file = chars.next().unwrap().to_ascii_lowercase();
        let rank = chars.next().unwrap();

        if file < 'a' || file > 'h' {
            return Err(ChessError::ParseSquare(string.to_owned())); } 
        if rank <'1' || rank > '8' {
            return Err(ChessError::ParseSquare(string.to_owned()));
        }

        Ok(Square::from((file as u8 - 'a' as u8) * 8 + rank as u8 - 1 ))
    }

}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Direction {
    Up = 8, 
    UpRight = 9, 
    Right = 1, 
    DownRight = -7,
    Down = -8, 
    DownLeft = -9, 
    Left = -1, 
    UpLeft = 7
}

impl Add<Direction> for Square {
    type Output = Square;
    fn add(self, adj: Direction) -> Square {
        let sq = self as i8;
        let adj = adj as i8;
        Square::from((sq + adj) as u8)
    }
}

pub struct Ray {
    square: i8,
    direction: Direction,
}

impl Iterator for Ray {
    type Item = Square;
    fn next(&mut self) -> Option<Square> {
        self.square += self.direction as i8;
        if self.square < 0 || self.square > 63 {
            return None;
        }
        Some(Square::from(self.square as u8))
    }
}

impl From<u8> for Square {
    fn from(t: u8) -> Square {
        unsafe {
            std::mem::transmute(t)
        }
    }
}

pub struct Squares {
    bitboard: Bitboard,
}

impl Iterator for Squares {
    type Item = Square;
    
    fn next(&mut self) -> Option<Square> {
        let zeros = self.bitboard.0.trailing_zeros() as u8;
        if zeros == 64 {
            return None;
        }
        self.bitboard.0 ^= 1 << zeros;
        Some(Square::from(8* (zeros / 8) + 7 - (zeros % 8)))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Display for Bitboard {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for rank in (0..8u8).rev() {
            for file in 0..8u8 {
                match self.at(Square::from(rank*8 + file)) {
                    true => s.push('O'),
                    false => s.push('.'),
                }
            }
            s.push('\n');
        }
        write!(f,"{}", s)
    }
}

impl Bitboard {

    pub fn empty() -> Bitboard {
        Bitboard(0)
    }

    pub fn squares(self) -> Squares {
        Squares { bitboard: self } 
    }

    pub fn flip_diagonal(self) -> Bitboard {
        let k1: u64 = 0x5500550055005500;
        let k2: u64 = 0x3333000033330000;
        let k4: u64 = 0x0f0f0f0f00000000;
        
        let mut x = self.0;

        let t  = k4 & (x ^ (x << 28));
        x ^=       t ^ (t >> 28) ;
        let t  = k2 & (x ^ (x << 14));
        x ^=       t ^ (t >> 14) ;
        let t  = k1 & (x ^ (x <<  7));
        x ^=       t ^ (t >>  7) ;
        Bitboard(x)
    }

    pub fn at(self, square: Square) -> bool {
        (self.0 & square.to_bitboard().0) != 0
    }

    pub fn set(&mut self, square: Square) {
        self.0 = self.0 | square.to_bitboard().0
    }

}
