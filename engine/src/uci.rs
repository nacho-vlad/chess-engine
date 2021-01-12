use std::io;
use chess::*;
use crate::*;
use std::collections::HashMap;

// fn make_moves(state: &Chessboard, moves: ) -> Chessboard {


// }

fn go_settings<'a>(input: &[&'a str]) -> HashMap<&'a str, i32> {
    let mut map = HashMap::new();

    for (subcmd, val) in input.iter().step_by(2).zip(input.iter().skip(1).step_by(2)) { 
        map.insert(*subcmd, val.parse().unwrap());
    } 
    map
}


pub fn run() {

    let mut chessboard = Chessboard::starting();
    loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        
        match input[0] {
            "uci" => {
                println!("id name Sah-O-Matic-2000");
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
                let settings = go_settings(&input[1..]);
                let depth = *settings.get("depth").unwrap_or(&6) as u32;
                let mov = best_move(&chessboard, depth);
                println!("bestmove {}",mov);
            },
            "isready" => {
                println!("readyok");   
            },
            _ => {}
        }
    }

}
