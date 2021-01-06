///This crate implements all of the rules of chess in an easy to use
///and performant way (benchmarks will come later). I wrote this crate
///for my very own chess engine.
pub mod position;
pub mod constants;
pub mod repr;

use thiserror::Error;

use crate::position::Position;
use crate::repr::*;
use std::sync::Arc;
use std::str::FromStr;



#[derive(Error, Debug)]
pub enum ChessError {
    #[error("{0} is not a valid square")]
    ParseSquare(String),
    #[error("{0}")]
    InvalidFEN(String),
    #[error("{0} is not a valid move")]
    ParseMove(String),
}


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Chessboard {
    pub position: Position,
    pub previous: Option<Arc<Chessboard>>
}

///Struct that represents the chessboard. This is the main struct
///of this crate.
impl Chessboard {
    
    pub fn new(position: Position) -> Chessboard {
        Chessboard {
            position,
            previous: None
        }
    }

    pub fn from_fen(fen: &str) -> Result<Chessboard, ChessError> {
        Ok(Chessboard {
            position: Position::from_fen(fen)?,
            previous: None
        })
    }

    pub fn starting() -> Chessboard {
        Chessboard {
            position: Position::starting(),
            previous: None,
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.position.legal_moves()
    }

    pub fn make_move(&self, legal_move: Move) -> Chessboard {
        Chessboard {
            position: self.position.make_move(legal_move),
            previous: Some(Arc::new(self.clone())),
        }
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.position.board.at(square)
    }

    pub fn parse_move(&self, mov: &str) -> Result<Move, ChessError> {
        if mov.len() != 4 && mov.len() != 5 {
            return Err(ChessError::ParseMove(mov.to_owned()))
        }
        
        let src = Square::from_str(&mov[0..2])?;
        let dst = Square::from_str(&mov[2..4])?;

        if mov.len() == 5 {
            let promotion = match mov.chars().collect::<Vec<char>>()[4] {
                'q' | 'Q' => Piece::Queen,
                'n' | 'N' => Piece::Knight,
                'b' | 'B' => Piece::Bishop,
                'r' | 'R' => Piece::Rook,
                _ => return Err(ChessError::ParseMove(mov.to_owned())),
            };
            return Ok(Move::Promotion(src,dst,promotion))
        }

        let piece = match self.piece_at(src) {
            None => return Err(ChessError::ParseMove(mov.to_owned())),
            Some(piece) => piece,
        };

        if let Some(sq) = self.position.en_passant {
            if dst == sq && piece == Piece::Pawn {
                return Ok(Move::EnPassant(src,dst));
            }
        }

        if piece == Piece::King {
            if src.file() as i8 - dst.file() as i8 == 2 {
                return Ok(Move::QueensideCastle);
            }
            if src.file() as i8 - dst.file() as i8 == -2 {
                return Ok(Move::KingsideCastle);
            }
        }

        Ok(Move::Normal(src,dst,piece)) 
    }

}








#[cfg(test)]
mod tests {
    #[test]
    fn perft_starting2() {
        use crate::position::Position;
        let position = Position::starting();
        assert_eq!(position.perft(2), 400);
    }
    #[test]
    fn perft_starting3() {
        use crate::position::Position;
        let position = Position::starting();
        assert_eq!(position.perft(3), 8902);
    }
    #[test]
    fn perft_starting4() {
        use crate::position::Position;
        let position = Position::starting();
        assert_eq!(position.perft(4), 197281);
    }
    #[test]
    fn perft_starting5() {
        use crate::position::Position;
        let position = Position::starting();
        assert_eq!(position.perft(5), 4865609);
    }
    #[test]
    fn perft_kiwipete2() {
        use crate::position::Position;
        let position = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(position.perft(2), 2039);
    }
    #[test]
    fn perft_kiwipete3() {
        use crate::position::Position;
        let position = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(position.perft(3), 97862);
    }
    #[test]
    fn perft_kiwipete4() {
        use crate::position::Position;
        let position = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(position.perft(4), 4085603);
    }
    #[test]
    fn perft_pos3_2() {
        use crate::position::Position;
        let position = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(position.perft(2), 191);
    }
    #[test]
    fn perft_pos3_3() {
        use crate::position::Position;
        let position = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(position.perft(3), 2812);
    }
    #[test]
    fn perft_pos3_4() {
        use crate::position::Position;
        let position = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(position.perft(4), 43238);
    }
    #[test]
    fn perft_pos3_5() {
        use crate::position::Position;
        let position = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(position.perft(5), 674624);
    }
    #[test]
    fn perft_pos4_2() {
        use crate::position::Position;
        let position = Position::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(position.perft(2), 264);
    }
    #[test]
    fn perft_pos4_3() {
        use crate::position::Position;
        let position = Position::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(position.perft(3), 9467);
    }
    #[test]
    fn perft_pos4_4() {
        use crate::position::Position;
        let position = Position::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(position.perft(4), 422333);
    }
    #[test]
    fn perft_pos5_2() {
        use crate::position::Position;
        let position = Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(position.perft(2), 1486);
    }
    #[test]
    fn perft_pos5_3() {
        use crate::position::Position;
        let position = Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(position.perft(3), 62379);
    }
    #[test]
    fn perft_pos5_4() {
        use crate::position::Position;
        let position = Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(position.perft(4), 2103487);
    }
    
}
