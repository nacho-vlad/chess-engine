from ..player.base import Player
import subprocess

class AI(Player):
    

    def __init__(self, path):

        self.engine = subprocess.Popen(path, stdout=subprocess.PIPE, stdin=subprocess.PIPE, text=True, bufsize=1)
        self.engine.stdin.write('uci\nisready\n')
        
        while True:
            line=self.engine.stdout.readline()
            if line == "readyok\n":
                break

    def get_move(self, moves):

        position = "position startpos "
        if len(moves)>0:
            position += "moves "

        moves = " ".join(moves)
        position += moves + '\n'

        self.engine.stdin.write(position)
        
        self.engine.stdin.write("go depth 6\n")
        
        while True:
            line=self.engine.stdout.readline().split()
            if line[0] == "bestmove":
                return line[1]
