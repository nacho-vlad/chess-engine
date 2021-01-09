use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use chess::{Chessboard, ChessError};

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
