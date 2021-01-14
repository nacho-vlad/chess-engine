use chess::repr::*;
use chess::*;

pub const MIN_SCORE: f64 = -10000.0;
pub const MAX_SCORE: f64 = 10000.0;


const POSITION_KING_EARLY : [f64;64] = [
    0.05,0.05,0.50,0.05,0.05,0.05,0.50,0.05,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.05,0.05,0.50,0.05,0.05,0.05,0.50,0.05,
];

const POSITION_KNIGHT : [f64;64] = [
    -0.50,0.00,0.00,0.00,0.00,0.00,0.00,-0.50,
    -0.30,0.00,0.00,0.00,0.00,0.00,0.00,-0.30,
    -0.30,0.00,0.00,0.00,0.00,0.00,0.00,-0.30,
    -0.30,0.00,0.00,0.00,0.00,0.00,0.00,-0.30,
    -0.30,0.00,0.00,0.00,0.00,0.00,0.00,-0.30,
    -0.30,0.00,0.00,0.00,0.00,0.00,0.00,-0.30,
    -0.30,0.00,0.00,0.00,0.00,0.00,0.00,-0.30,
    -0.50,0.00,0.00,0.00,0.00,0.00,0.00,-0.50,
];

const POSITION_PAWN : Colored<[f64;64]> = Colored([
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,
    0.02,0.02,0.02,0.02,0.02,0.02,0.02,0.02,
    0.03,0.03,0.03,0.03,0.03,0.03,0.03,0.03,
    0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,
    0.05,0.05,0.05,0.05,0.05,0.05,0.05,0.05,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
],[
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.05,0.05,0.05,0.05,0.05,0.05,0.05,0.05,
    0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,
    0.03,0.03,0.03,0.03,0.03,0.03,0.03,0.03,
    0.02,0.02,0.02,0.02,0.02,0.02,0.02,0.02,
    0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
    0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,
]);



pub fn game_over(chessboard: &Chessboard) -> Option<f64> {
    if let Some(result) = chessboard.game_result() {
        match result {
            None => Some(0.0),
            Some(color) => match color == chessboard.position.turn {
                true => Some(MAX_SCORE),
                false => Some(MIN_SCORE),
            } 
        }
    } else {
    None
    }
}


pub fn evaluate_color(chessboard: &Chessboard, color: Color) -> f64 {
    let mut score = 0.0;
    let board = chessboard.position.board;


    for sq in board[color][Piece::Pawn].squares() {
        score += 1.0 + POSITION_PAWN[color][sq as usize];
    }
    for sq in board[color][Piece::Knight].squares() {
        score += 3.0 + POSITION_KNIGHT[sq as usize];
    }

    score += 3.0 * board[color][Piece::Bishop].count() as f64;
    score += 5.0 * board[color][Piece::Rook].count() as f64;
    score += 9.0 * board[color][Piece::Queen].count() as f64;
    
    if score > 10.0 {
        let king_sq = board[color][Piece::King].squares().next().unwrap();
        score += POSITION_KING_EARLY[king_sq as usize]
    }

    let attacked = board.attacks(color);
    score += 0.03 * attacked.count() as f64; 
    
    score
}



pub fn evaluate(chessboard: &Chessboard) -> f64 { 
    
     // if let Some(score) = game_over(chessboard) {
     //    return score;
     // }
     
     let player = chessboard.position.turn;
     let opponent = player.other();
    
     let score = evaluate_color(chessboard,player) - evaluate_color(chessboard,opponent);

     score
}
