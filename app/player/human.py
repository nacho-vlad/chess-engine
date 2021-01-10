from ..player.base import Player
from app.ui.base import UI

class Human(Player):

    def __init__(self, ui):
        self.ui = ui
        super().__init__()

    def get_move(self, moves):
        return self.ui.get_move()
        

