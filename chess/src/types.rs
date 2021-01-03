use crate::pieces::Pieces;
use crate::bitboard::{Bitboard, Square};
use std::ops::{Index, IndexMut};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessError {
    #[error("{0} is not a valid square")]
    ParseSquare(String),
    #[error("{0}")]
    InvalidFEN(String),
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

#[derive(Debug, Copy,Clone,Hash,PartialEq,Eq)]
pub struct Move {
    source: Square,
    destination: Square, 
}

