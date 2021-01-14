pub mod uci;
pub mod evaluation;

use evaluation::*;
use chess::*;
use chess::repr::Move;
use rayon::prelude::*;
use std::sync::{
    Arc,
    Mutex,
};


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
