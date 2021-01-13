from app.rustchess import Chessboard

import unittest


class GameTest(unittest.TestCase):

    def setUp(self):
        self.board = Chessboard()

    def test_move(self):

        self.board.make_move("e2e4")
        self.assertTrue(self.board.at(4,5) == "P")
        self.assertTrue(self.board.at(2,5) == None)
        try:
            self.board.make_move("e4e6")
            self.assertTrue(False)
        except ValueError as e:
            pass

    def test_result(self):
        self.assertTrue(self.board.game_result() == None)
    
    def test_turn(self):
        
        self.assertTrue(self.board.turn() == "white")
        self.board.make_move("e2e4")
        self.assertTrue(self.board.turn() == "black")
    




if __name__ == "__main__":
    unittest.main()
