import abc

class UI(abc.ABC):

    @abc.abstractmethod
    def draw(self, board):
        pass

    @abc.abstractmethod
    def get_move(self):
        pass
     


