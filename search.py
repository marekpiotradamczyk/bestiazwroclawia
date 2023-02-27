from multiprocessing import Process
from multiprocessing.connection import Connection
from parser_commands import QuitCommand
from search_commands import *
from mcts import MCST
import random
import chess
import logging
from time_management import TimeManager

class SearchProcess(Process):
    def __init__(self, connection : Connection):
        Process.__init__(self)
        #two way
        self.connection = connection
        self.position = None
        self.best_move = None
        self.mcts = None
        self.searching = False
        self.time_manager = TimeManager()
        logging.info("Search process ready!")
    
    def run(self):
        while True:
            # logging.debug("connection poll")
            if self.connection.poll():
                msg = self.connection.recv()
                match msg:
                    case QuitCommand():
                        break
                    case SearchPositionCommand():
                        self._handle_searchposition_command(msg)
                    case StopSearchCommand():
                        self._handle_stopsearch_command()
            # logging.debug("Searching part")
            if self.searching:
                if self.time_manager.out_of_time():
                    self._handle_stopsearch_command()
                else:
                    self.mcts.run_iteration()


    def _handle_searchposition_command(self, command : SearchPositionCommand):
        self.searching = True
        self.position = command.position
        self.time_manager.set_timer(command.t, command.infinite)
        self.mcts = MCST(self.position)

    def _handle_stopsearch_command(self):
        self.searching = False
        self._find_best_move()
        self.connection.send(BestMoveCommand(move=self.best_move))

    def _find_best_move(self):
        # self.best_move = random.choice(list(self.position.legal_moves))
        self.best_move = self.mcts.get_best_move()
        # moves = list(self.position.legal_moves)
        # random.shuffle(moves)
        # self.position.push(moves[0])
        # be, self.best_move = _evaluate_board(self.position), moves[0]
        # self.position.pop()
        # for m in moves:
        #     self.position.push(m)
        #     e = _evaluate_board(self.position)
        #     if e < be:
        #         be = e
        #         self.best_move = m
        #     self.position.pop()
        
