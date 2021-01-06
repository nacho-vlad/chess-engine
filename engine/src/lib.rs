pub mod uci;

use chess::*;
use chess::repr::{Move, Piece};
use constants::STARTING_POS_FEN;



fn evaluate(chessboard: &Chessboard) -> f64 {
     let player = chessboard.position.turn;
     let opponent = player.other();
     let board = chessboard.position.board;
    
     let mut score: f64 = 0.0;
     score += 1.0 * (board[player][Piece::Pawn].count() as f64
                    -board[opponent][Piece::Pawn].count() as f64);
     score += 3.0 * (board[player][Piece::Bishop].count() as f64
                    -board[opponent][Piece::Bishop].count() as f64);
     score += 3.0 * (board[player][Piece::Knight].count() as f64
                    -board[opponent][Piece::Knight].count() as f64);
     score += 5.0 * (board[player][Piece::Rook].count() as f64
                    -board[opponent][Piece::Rook].count() as f64);
     score += 9.0 * (board[player][Piece::Queen].count() as f64
                    -board[opponent][Piece::Queen].count() as f64);

     score
}


fn negamax(position: &Chessboard, depth: u32) ->  f64 {
    if depth == 0 {
        return evaluate(position);
    }

    let legal_moves = position.legal_moves();

    if legal_moves.len() == 0 {
        match position.position.in_check() {
            true => return -1000.0, 
            false => return 0.0,
        }
    }

    let mut best_score: f64 = -10000.0;
    for mv in legal_moves {
        best_score = best_score.max(-negamax(&position.make_move(mv), depth-1));
    }
    return best_score
}


fn best_move(chessboard: &Chessboard, depth: u32) -> Move {
    let legal_moves = chessboard.legal_moves();

    let mut best_score = -10000.0;
    let mut mov = None;
    for mv in legal_moves {
        let score = -negamax(&chessboard.make_move(mv), depth-1);
        if best_score < score {
            best_score = score;
            mov = Some(mv);
        }
    }
    mov.unwrap()
}
