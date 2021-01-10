use pyo3::prelude::*;
use chess::{Chessboard, ChessError};
use chess::repr::*;

struct PyChessError(pub ChessError);

impl std::convert::From<ChessError> for PyChessError {
    fn from(err: ChessError) -> PyChessError {
        PyChessError(err)
    }
}


impl std::convert::From<PyChessError> for PyErr {
    fn from(err: PyChessError) -> PyErr {
        use pyo3::exceptions::PyValueError;
        PyValueError::new_err(err.0.to_string())
    }

}

#[pyclass(name="Chessboard")]
struct PyChessboard {
    board: Chessboard,
}

#[pymethods]
impl PyChessboard { 
    #[new]
    fn new() -> Self {
        PyChessboard {
            board: Chessboard::starting()
        }
    }
    
    fn to_ascii(&self) -> String {
        self.board.position.to_ascii()
    }

    fn turn(&self) -> String {
        match self.board.position.turn {
            Color::White => "white".to_owned(),
            Color::Black => "black".to_owned()
        }
    }

    fn at(&self, rank: u8, file: u8) -> Option<char> {
        let square = Square::from((rank-1)*8 + file-1);
        
        Some(match self.board.at(square)? {
            (Color::White, Piece::Pawn) => 'P',
            (Color::White, Piece::Bishop) => 'B',
            (Color::White, Piece::Knight) => 'N',
            (Color::White, Piece::Queen) => 'Q',
            (Color::White, Piece::King) => 'K',
            (Color::White, Piece::Rook) => 'R',
            (Color::Black, Piece::Pawn) => 'p',
            (Color::Black, Piece::Bishop) => 'b',
            (Color::Black, Piece::Knight) => 'n',
            (Color::Black, Piece::Rook) => 'r',
            (Color::Black, Piece::Queen) => 'q',
            (Color::Black, Piece::King) => 'k',
        })
    }

    fn game_result(&self) -> Option<String> {
        let result = self.board.game_result()?;
        Some(match result {
            None => "draw".to_string(),
            Some(Color::White) => "white".to_string(),
            Some(Color::Black) => "black".to_string(),
        })
    }

    fn undo(&mut self) -> Result<(), PyChessError> {
        self.board = self.board.previous()?;
        self.board = self.board.previous()?;
        Ok(())
    }

    fn make_move(&mut self, mov: &str) -> Result<(), PyChessError> {
        self.board = self.board.make_str_move(mov)?;
        Ok(())
    }
}

#[pymodule]
fn rustchess(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyChessboard>()?;

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
