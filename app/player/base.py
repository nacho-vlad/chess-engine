import abc

class Player(abc.ABC):
    """Class that represents a player of the game,
    can be either human or AI.
    """
    @abc.abstractmethod
    def get_move(moves):
        """Get a move in long algebraic notation

        Parameters
        ----------
        moves : list[str]
            the list of moves until now
        
        Returns
        -------
        move : str
            chess move in long algebraic notation
        """
        pass
