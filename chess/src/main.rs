use chess::position::Position;
use chess::constants::*;



fn main() {
    let position = Position::from_fen("r3k2r/p1ppqNb1/1n2pnp1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 2").unwrap();
    // println!("{}", position.to_ascii());
    // let moves = position.legal_moves();
    // let new_positions: Vec<Position> = moves.iter().map(|mv| position.apply_move(*mv)).collect();
    // for np in new_positions.iter() {
    //     println!("{}", np.to_ascii());
    // }
    // println!("{}", KNIGHT_ATTACKS[chess::repr::Square::A8 as usize]);
    println!("{}", position.perft(2))
}
