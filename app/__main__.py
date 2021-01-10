from app.game import Game
from app.player.human import Human
from app.player.ai import AI
from app.ui.cli import CLI
from app.ui.gui import GUI

def main():

    ui = GUI()
    white = AI("./stockfish/stockfish")
    black = AI("./stockfish/stockfish")
    # white = Human(ui)
    # black = Human(ui)

    game = Game(white, black, ui)
    game.play()


if __name__ == "__main__":
    main()
