pub mod uci;

use chess::*;
use chess::repr::{Move, Piece};
use rayon::prelude::*;
use std::sync::{
    Arc,
    Mutex,
};

const MIN_SCORE: f64 = -10000.0;
const MAX_SCORE: f64 = 10000.0;

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

// fn negamax(position: &Chessboard, should_stop: Arc<AtomicBool>, alpha: f64, beta: f64, depth: u32) ->  f64 {
    
//     if should_stop.load(Ordering::Relaxed) {
//         return MIN_SCORE
//     }

//     if depth == 0 {
//         return evaluate(position)
//     }

//     let legal_moves = position.legal_moves();

//     if legal_moves.len() == 0 {
//         match position.position.in_check() {
//             true => return -1000.0, 
//             false => return 0.0,
//         }
//     }

//     let alpha = Arc::new(Mutex::new(alpha.max(MIN_SCORE)));
//     let cut_off = Arc::new(AtomicBool::new(false));

//     legal_moves.par_iter().for_each(|mv| {
//         let cut_off = Arc::clone(&cut_off); 
        
//         if should_stop.load(Ordering::Relaxed) {
//             cut_off.store(true, Ordering::Relaxed);
//             return
//         }

//         if cut_off.load(Ordering::Relaxed) {
//             return
//         }

//         let score = {
//             let alpha = *Arc::clone(&alpha).lock().unwrap();
//             -negamax(&position.make_move(*mv), Arc::clone(&cut_off), -beta, -alpha, depth-1)
//         };

//         let alpha = Arc::clone(&alpha);
//         let mut alpha = alpha.lock().unwrap();

//         *alpha = alpha.max(score);
//         if *alpha >= beta {
//             cut_off.store(true, Ordering::Relaxed);
//         }
//     });

//     let best_score = *alpha.lock().unwrap();
//     best_score
// }

fn negamax(position: &Chessboard, alpha: f64, beta: f64, depth: u32) ->  f64 {
    
    if depth == 0 {
        return evaluate(position)
    }

    let legal_moves = position.legal_moves();

    if legal_moves.len() == 0 {
        match position.position.in_check() {
            true => return -1000.0, 
            false => return 0.0,
        }
    }

    let mut alpha = alpha.max(MIN_SCORE);

    for mv in legal_moves {
        alpha = alpha.max(-negamax(&position.make_move(mv), -beta, -alpha, depth-1));
        if alpha >= beta {
            break;
        }
    }

    alpha
}


pub fn best_move(chessboard: &Chessboard, depth: u32) -> Move {
    let legal_moves = chessboard.legal_moves();

    let best_score = Arc::new(Mutex::new(MIN_SCORE));
    let mov = Arc::new(Mutex::new(None));

    let beta = MAX_SCORE;

    legal_moves.par_iter().for_each(|mv| {

        let alpha = *Arc::clone(&best_score).lock().unwrap();

        let score = -negamax(&chessboard.make_move(*mv), -beta, -alpha, depth-1);

        let best_score = Arc::clone(&best_score);
        let mut best_score = best_score.lock().unwrap();

        let mov = Arc::clone(&mov);
        let mut mov = mov.lock().unwrap();

        if *best_score < score {
            *best_score = score;
            *mov = Some(*mv);
        }
    });
    let mov = *mov.lock().unwrap();
    mov.unwrap()
}
