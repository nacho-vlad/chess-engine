use std::io;
use chess::*;
use crate::*;


// fn make_moves(state: &Chessboard, moves: ) -> Chessboard {


// }



pub fn run() {

    let mut chessboard = Chessboard::starting();
    loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        
        match input[0] {
            "uci" => {
                println!("id name Sah-o-mat");
                println!("id author Florin-Vlad Sabau");
                println!("uciok");
            },
            "position" => {
                let mut moves = Vec::new();
                if input[1] == "startpos" {
                    chessboard = Chessboard::starting(); 
                    if input.len() > 3 {
                        moves = input[3..].iter().collect();
                    }
                } else {
                    chessboard = Chessboard::from_fen(&input[2..8].join(" ")).unwrap();
                    if input.len() > 9 {
                        moves = input[9..].iter().collect();
                    }
                }

                for mv in moves.iter() {
                    let mv = chessboard.parse_move(mv).unwrap();
                    chessboard = chessboard.make_move(mv);
                }
                
            },
            "go" => {
                let mov = best_move(&chessboard, 4);
                println!("bestmove {}",mov);
            },
            "isready" => {
                println!("readyok");   
            },
            _ => {}
        }
    }

}
