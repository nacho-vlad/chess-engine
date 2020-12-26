use crate::bitboard::{
    Bitboard,
    Square
};
use crate::pieces::{
    Piece,
    Pieces,
};
use std::ops::{
    Index,
    IndexMut,
};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::Arc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


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




impl Colored<Pieces> {
    pub fn occupied(&self) -> Bitboard {
        Bitboard(self[Color::White].occupied().0 | self[Color::Black].occupied().0)
    }
    
    pub fn unoccupied(&self) -> Bitboard { 
        Bitboard(self[Color::White].unoccupied().0 | self[Color::Black].unoccupied().0)
    }
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, EnumIter)]
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CastlingRights {
    kingside: bool,
    queenside: bool,
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    board: Colored<Pieces>,
    turn: Color,
    castling_rights: Colored<CastlingRights>,
    en_passant: Option<Square>,
    half_moves: u16,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Chessboard {
    position: Position,
    previous: Option<Arc<Chessboard>>
}


impl Position {
     
    pub fn from_fen(fen: &str) -> Position {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        
        let mut board = Colored(Pieces::empty(), Pieces::empty());
        let mut sq = Square::A8 as i8;
        for c in parts[0].chars() {
            match c {
                'P' => board[Color::White][Piece::Pawn].set(Square::from(sq as u8)),
                'N' => board[Color::White][Piece::Knight].set(Square::from(sq as u8)),
                'B' => board[Color::White][Piece::Bishop].set(Square::from(sq as u8)),
                'R' => board[Color::White][Piece::Rook].set(Square::from(sq as u8)),
                'Q' => board[Color::White][Piece::Queen].set(Square::from(sq as u8)),
                'K' => board[Color::White][Piece::King].set(Square::from(sq as u8)),
                'p' => board[Color::Black][Piece::Pawn].set(Square::from(sq as u8)),
                'n' => board[Color::Black][Piece::Knight].set(Square::from(sq as u8)),
                'b' => board[Color::Black][Piece::Bishop].set(Square::from(sq as u8)),
                'r' => board[Color::Black][Piece::Rook].set(Square::from(sq as u8)),
                'q' => board[Color::Black][Piece::Queen].set(Square::from(sq as u8)),
                'k' => board[Color::Black][Piece::King].set(Square::from(sq as u8)),
                '/' => sq -= 17, 
                d if '1' <= d && d <= '9' => sq += (d as u8 - '0' as u8 - 1) as i8,
                _ => panic!("Invalid pieces FEN")
            }
            sq += 1;
        }

        let turn = match parts[1] {
            "w" | "W" => Color::White,
            "b" | "B" => Color::Black,
            _ => panic!("Invalid turn FEN"),
        };
        
        let mut white_kingside = false;
        let mut white_queenside = false;
        let mut black_kingside = false;
        let mut black_queenside = false;

        for c in parts[2].chars() {
            match c {
                'K' => white_kingside = true,
                'Q' => white_queenside = true,
                'k' => black_kingside = true,
                'q' => black_queenside = true,
                '-' => {},
                _ => panic!("Invalid castling rights FEN")
            }
        }

        let castling_rights = Colored(CastlingRights::new(white_kingside, white_queenside),
                                      CastlingRights::new(black_kingside, black_queenside));

        let en_passant = match parts[3] {
            "-" => None,
            sq => Some(Square::from_str(sq).unwrap())
        };

        let half_moves = str::parse::<u16>(parts[4]).unwrap();

        Position { board, turn, castling_rights, en_passant, half_moves}
    }


    pub fn to_unicode(&self) -> String {

        let mut board: String = String::new();
        
        for row in (0..8u8).rev() {
            for col in 0..8u8 {
                for color in Color::iter() {
                    let sq = match self.board[color].at(Square::from(row*8+col)) {
                        None => " ".to_owned(),
                        Some(piece) => piece.to_unicode(color).to_string(),
                    };
                    board.push_str(&sq);
                }
            }
            board.push('\n');
        }
        board
    }
}

