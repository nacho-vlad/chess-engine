use chess::Chessboard;
use chess::constants::*;



fn main() {
    let chessboard = Chessboard::from_fen("r3k2r/p1ppqNb1/1n2pnp1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 2").unwrap();
    println!("{}", chessboard.position.to_ascii());
    let moves = chessboard.legal_moves();
    let new_positions: Vec<Chessboard> = moves.iter().map(|mv| chessboard.make_move(*mv)).collect();
    for np in new_positions.iter() {
        println!("{}", np.previous.as_ref().unwrap().position.to_ascii());
    }
    println!("{}", chessboard.position.perft(2))
}
