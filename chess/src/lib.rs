pub mod bitboard;
pub mod position;
pub mod pieces;
pub mod constants;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessError {
    #[error("{0} is not a valid square")]
    ParseSquare(String),
}







#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
