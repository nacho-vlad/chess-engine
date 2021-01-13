import abc

class UI(abc.ABC):
    """Abstract class that represents a UI."""

    @abc.abstractmethod
    def draw(self, board):
        """Display the board for the users.
        
        Parameters
        ----------
        board : Chessboard
            the board to be displayed
        """
        pass

    @abc.abstractmethod
    def get_move(self):
        """Get a move from the user.
        
        Returns
        -------
        move : str
            move in long algebraic notation.
        """

        pass
     

    @abc.abstractmethod
    def game_over(self,result):
        """Display game over message."""
        pass


