from ..ui.base import UI
import pygame
from pygame.locals import *

def to_coords(file, rank):
    return ((file-1)*80, (8-rank)*80)


def from_coords(x,y):
    file = min(8,x//80+1)
    rank = min(8,8-y//80)
    return (file, rank)

def square(file,rank):
    return chr(file-1 + ord('a')) + chr(rank + ord('0'))

class GUI(UI):

    def __init__(self):
        pygame.init()
        self.screen = pygame.display.set_mode((640,640))
        pygame.display.set_caption('Chess')

        self.background = pygame.Surface(self.screen.get_size()).convert()
        self.selected = None
        self.board = None
        self.draw_background()
    
    def draw_background(self):
         
        WHITE = (240,217,181)
        BLACK = (181,136,99)

        for file in range(1,9):
            for rank in range(1,9):
                color = BLACK if (file+rank)%2==0 else WHITE
                coords = to_coords(file,rank)
                pygame.draw.rect(self.background,color,(coords[0], coords[1], 80, 80))

    def draw_pieces(self, board):
        
        for file in range(1,9):
            for rank in range(1,9):
                piece = board.at(rank,file)
                if piece is None:
                    continue

                name = piece.lower()
                if piece.isupper():
                    name += 'l' 
                else:
                    name += 'd'
                name = './images/Chess_' + name + 't60.png'
                piece_img = pygame.image.load(name)
                coords = to_coords(file,rank)
                self.screen.blit(piece_img, (coords[0] + 10, coords[1] + 10))

    def draw(self, board):
        self.screen.blit(self.background, (0,0))
        self.board = board
        self.draw_pieces(board)
        
        selected_color = {
                'white': (139,233,253),
                'black': (255,121,198)
            }[board.turn()]

        if self.selected is not None:
            coords = to_coords(*self.selected)
            pygame.draw.rect(self.screen, selected_color , (coords[0], coords[1], 80, 80), 5)    

        pygame.display.flip()

    def get_move(self):
        
        self.selected = None
        while True:
            for e in pygame.event.get():
                if e.type == pygame.MOUSEBUTTONDOWN:
                    (mouseX, mouseY) = pygame.mouse.get_pos()

                    left_click = e.button == 1
                    
                    if self.selected is not None and left_click:
                        return square(*self.selected) + square(*from_coords(mouseX,mouseY)) 

                    self.selected = from_coords(mouseX,mouseY) if left_click else None

                if e.type == pygame.QUIT:
                    return "quit"

                if e.type == pygame.KEYDOWN:
                    if e.key == pygame.K_SPACE:
                        return "undo"
            self.draw(self.board)


    def game_over(self, result):
        
        while True:
            for e in pygame.event.get():
                if e.type == pygame.QUIT:
                    return
        
