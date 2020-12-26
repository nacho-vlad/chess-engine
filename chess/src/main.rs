use chess::pieces::Pieces;
use chess::position::Position;
use chess::bitboard::{Bitboard, Square};
use chess::constants::*;


fn main() {
    let position = Position::from_fen("rnbqkbnr/pppppppp/1p6/1P6/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", position.to_unicode());
    println!("{}", SLIDE_HORIZONTAL[Square::H2 as usize][&0b00100001u8]);
    println!("{}", SLIDE_VERTICAL[&(Square::A2, Bitboard(0x01_01_00_00_01_00_01_00))]);
}
