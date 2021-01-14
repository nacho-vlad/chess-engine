from app.game import Game
from app.player.human import Human
from app.player.ai import AI
from app.ui.cli import CLI
from app.ui.gui import GUI
import yaml

def main():
    settings = yaml.load(open("config.yaml"))
    
    ui = {
            'gui': lambda: GUI(),
            'cli': lambda: CLI(),
        }[settings['ui']]()
    
    
    white = None
    if settings['white']['type'] == 'human':
        white = Human(ui)
    elif settings['white']['type'] == 'ai':
        white = AI(settings['white']['path'], settings['white']['depth'])
    
    black = None
    if settings['black']['type'] == 'human':
        black = Human(ui)
    elif settings['black']['type'] == 'ai':
        black = AI(settings['black']['path'], settings['black']['depth'])


    game = Game(white, black, ui)
    game.play()


if __name__ == "__main__":
    main()
