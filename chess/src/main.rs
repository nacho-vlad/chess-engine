use chess::Chessboard;
use chess::constants::*;



fn main() {
    let mut chessboard = Chessboard::from_fen(STARTING_POS_FEN).unwrap();

    for pos in chessboard.history() {
        println!("{}", pos.to_ascii())  
    }
}
