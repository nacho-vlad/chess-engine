from ..ui.base import UI
import subprocess

class TUI(UI):

    def __init__(self):
        super().__init__()

    def draw(self, board):
        subprocess.run("clear")
        print(board.to_ascii())

    def get_move(self, moves):

        color = "blue" if len(moves)%2==0 else "pink"

        move = input("Enter a move for " + color + ":")
        return move


