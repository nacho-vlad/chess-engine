use crate::repr::{
    Color,
    bitboard::{Bitboard, Square}, 
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::ops::{Index,IndexMut};


#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl Piece {
    pub fn to_ascii(self) -> char {

        match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }

    pub fn to_unicode(self, color: Color) -> char {
        match color {
            Color::White => match self {
                Piece::Pawn => '♙',
                Piece::Knight => '♘',
                Piece::Bishop => '♗',
                Piece::Rook => '♖',
                Piece::Queen => '♕',
                Piece::King => '♔',
            },
            Color::Black => match self {
                Piece::Pawn => '\u{265F}',
                Piece::Knight => '♞',
                Piece::Bishop => '♝',
                Piece::Rook => '♜',
                Piece::Queen => '♛',
                Piece::King => '♚',
            }

        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Pieces {
    pieces: [Bitboard; 6],
}

impl Pieces {
    
    pub fn empty() -> Pieces {
        Pieces {
            pieces: [Bitboard::empty(),
                     Bitboard::empty(),
                     Bitboard::empty(),
                     Bitboard::empty(),
                     Bitboard::empty(),
                     Bitboard::empty()], 
        }
    }

    pub fn at(&self, square: Square) -> Option<Piece> {
        for piece in Piece::iter() {
            if self[piece].at(square) {
                return Some(piece);
            }
        }
        None
    }

    pub fn unset(&mut self, square: Square) {
        for piece in Piece::iter() {
            self[piece].unset(square);
        }
    }

    pub fn occupied(&self) -> Bitboard { 
        self[Piece::Pawn] | self[Piece::Bishop] | self[Piece::Knight] |
        self[Piece::Rook] | self[Piece::Queen]  | self[Piece::King]
    }

    pub fn unoccupied(&self) -> Bitboard {
        Bitboard(!self.occupied().0)
    }
}


impl Index<Piece> for Pieces {
    type Output = Bitboard;
    
    fn index(&self, piece: Piece) -> &Bitboard {
        &self.pieces[piece as usize]
    }
}

impl IndexMut<Piece> for Pieces {
    
    fn index_mut(&mut self, piece: Piece) -> &mut Bitboard {
        &mut self.pieces[piece as usize]
    }
}

