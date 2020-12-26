use chess::pieces::Pieces;
use chess::position::Position;
use chess::bitboard::{Bitboard, Square};


fn main() {
    let position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{:x?}", position);
    println!("{}", position.to_unicode());
}
