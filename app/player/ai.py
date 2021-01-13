from ..player.base import Player
import subprocess

class AI(Player):
    """Player played by a UCI compatible chess engine.
    It spawns the engine as a subprocess

    Parameters
    ----------
    path : str
        the path to the executable
    depth : int
        the maximum depth for the search
    """

    def __init__(self, path, depth):

        self.engine = subprocess.Popen(path, stdout=subprocess.PIPE, stdin=subprocess.PIPE, text=True, bufsize=1)
        self.engine.stdin.write('uci\nisready\n')
        self.depth = depth 
        
        while True:
            line=self.engine.stdout.readline()
            if line == "readyok\n":
                break

    def get_move(self, moves):
        """Gets the move from the engine, following the 
        UCI protocol.
        """
        position = "position startpos "
        if len(moves)>0:
            position += "moves "

        moves = " ".join(moves)
        position += moves + '\n'

        self.engine.stdin.write(position)
        
        self.engine.stdin.write(f"go depth {self.depth}\n")
        
        while True:
            line=self.engine.stdout.readline().split()
            if line[0] == "bestmove":
                return line[1]
