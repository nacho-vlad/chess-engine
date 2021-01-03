use chess::pieces::Pieces;
use chess::position::Position;
use chess::types::Color;
use chess::bitboard::{Bitboard, Square};
use chess::constants::*;


fn perft(position: &Position, depth: u32) -> u32 {
    let moves = position.legal_moves();
    let mut nodes = 0;
    
    // println!("{}", position.to_ascii());
    if depth == 1 {
        return moves.len() as u32
    }

    for mv in moves.iter() {
        let new_nodes = perft(&position.apply_move(*mv), depth-1);
        // if depth == 1 {
        //     println!("{:?}: {}", mv, new_nodes);
        // }
        nodes += new_nodes
    }

    nodes
}

fn main() {
    let position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    // println!("{}", position.to_ascii());
    // let moves = position.legal_moves();
    // let new_positions: Vec<Position> = moves.iter().map(|mv| position.apply_move(*mv)).collect();
    // for np in new_positions.iter() {
    //     println!("{}", np.to_ascii());
    // }
    // println!("{:?}", position.legal_moves());
    println!("{}", perft(&position, 6))
}
