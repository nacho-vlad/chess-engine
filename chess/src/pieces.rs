use crate::bitboard::{Bitboard, Square};
use crate::position::Color;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::ops::{Index,IndexMut};


#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl Piece {
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

    pub fn occupied(&self) -> Bitboard { 
        Bitboard(self[Piece::Pawn].0 | self[Piece::Bishop].0 | self[Piece::Knight].0 |
                 self[Piece::Rook].0 | self[Piece::Queen].0  | self[Piece::King].0 )
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
