use chess::pieces::Pieces;
use chess::position::Position;
use chess::bitboard::{Bitboard, Square};
use chess::constants::*;


fn main() {
    let position = Position::from_fen("rnbqkbnr/pppppppp/1p6/1P6/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", position.to_unicode());
    println!("{}", SLIDE_HORIZONTAL[&(Square::A1, Bitboard(4))].1);
    println!("{}", SLIDE_VERTICAL[&(Square::A2, Bitboard(0x01_01_00_00_01_00_01_00))].0);
    println!("{}", SLIDE_MAIN_DIAGONAL[&(Square::F2, Bitboard(0x00_00_02_04_00_00_00_00))].0);
}
