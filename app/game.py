from app.rustchess import Chessboard

class Game:

    def __init__(self, white, black, ui):
        self.white = white
        self.black = black
        self.ui = ui

    def play(self):

        moves = []
        board = Chessboard()
        white_turn = True 
        
        while True:
            try:
                self.ui.draw(board)

                move = self.white.get_move(moves) if white_turn else self.black.get_move(moves)
                try:
                    board.make_move(move)
                    white_turn = not white_turn 
                    moves.append(move)
                except Exception:
                    pass
            except KeyboardInterrupt:
                return







