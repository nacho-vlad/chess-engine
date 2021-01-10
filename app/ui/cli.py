from ..ui.base import UI
import subprocess

class CLI(UI):

    def __init__(self):
        super().__init__()

    def draw(self, board):
        subprocess.run("clear")
        print(board.to_ascii())

    def get_move(self, moves):

        color = "blue" if len(moves)%2==0 else "pink"

        move = input("Enter a move for " + color + ":\n")
        return move

    def game_over(self, result):

        message = {
                'white': "Blue has won",
                'black': "Pink has won",
                'draw': "Draw",
            }[result]
        print(message)

