from multiprocessing import Process
from multiprocessing.connection import Connection
from parser_commands import QuitCommand
from search_commands import *
import random
import chess

def _evaluate_board(board : chess.Board) -> float:
    on_move = board.turn
    not_on_move = chess.WHITE if on_move == chess.BLACK else chess.BLACK
    nr_p_on = lambda x: len(board.pieces(x, on_move))
    nr_p = lambda x: len(board.pieces(x, not_on_move))
    if board.is_checkmate():
        return -1000.0
    if board.is_stalemate():
        return 0.0
    score = 0.0
    score += nr_p_on(chess.PAWN)
    score += 3.0*nr_p_on(chess.KNIGHT)
    score += 3.5*nr_p_on(chess.BISHOP)
    score += 4.0*nr_p_on(chess.ROOK)
    score += 9.0*nr_p_on(chess.QUEEN)
    score -= nr_p(chess.PAWN)
    score -= 3.0*nr_p(chess.KNIGHT)
    score -= 3.5*nr_p(chess.BISHOP)
    score -= 4.0*nr_p(chess.ROOK)
    score -= 9.0*nr_p(chess.QUEEN)
    return score

class SearchProcess(Process):
    def __init__(self, connection : Connection):
        Process.__init__(self)
        #two way
        self.connection = connection
        self.position = None
        self.best_move = None
    
    def run(self):
        while True:
            if self.connection.poll():
                msg = self.connection.recv()
                match msg:
                    case QuitCommand():
                        break
                    case SearchPositionCommand():
                        self._handle_searchposition_command(msg)
                    case StopSearchCommand():
                        self._handle_stopsearch_command()

    def _handle_searchposition_command(self, command : SearchPositionCommand):
        self.position = command.position
        self._find_best_move()

    def _handle_stopsearch_command(self):
        self.connection.send(BestMoveCommand(move=self.best_move))

    def _find_best_move(self):
        moves = list(self.position.legal_moves)
        random.shuffle(moves)
        self.position.push(moves[0])
        be, self.best_move = _evaluate_board(self.position), moves[0]
        self.position.pop()
        for m in moves:
            self.position.push(m)
            e = _evaluate_board(self.position)
            if e < be:
                be = e
                self.best_move = m
            self.position.pop()
        
