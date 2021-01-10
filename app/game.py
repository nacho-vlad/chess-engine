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

                result = board.game_result()
                if result is not None:
                    self.ui.game_over(result)
                    break

                move = self.white.get_move(moves) if white_turn else self.black.get_move(moves)
                if move == 'quit':
                    break
                
                try:
                    if move == 'undo':
                        board.undo()
                        continue
                    try:
                        board.make_move(move)
                    except Exception:
                        board.make_move(move+'q')

                    white_turn = not white_turn 
                    moves.append(move)
                except Exception:
                    pass
            except KeyboardInterrupt:
                return







