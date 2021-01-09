from app.game import Game
from app.player.human import Human
from app.player.ai import AI
from app.ui.tui import TUI

def main():

    tui = TUI()
    white = AI("./stockfish/stockfish")
    black = AI("./stockfish/stockfish")

    game = Game(white, black, tui)
    game.play()


if __name__ == "__main__":
    main()
