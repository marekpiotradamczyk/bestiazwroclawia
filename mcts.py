import chess
import random
import math
import logging

_piece_values = {chess.PAWN : 1.0, chess.KNIGHT : 3.0, chess.BISHOP : 3.5, chess.ROOK : 5.0, chess.QUEEN : 9.0}

def _evaluate_board(board : chess.Board) -> float:
    score = 0.0
    outcome = board.outcome(claim_draw=True)
    if outcome != None:
        if outcome.winner == None:
            return 0.0
        if outcome.winner == board.turn:
            return 10.0
        return -10.0
    for p, v in _piece_values.items():
        score += v * len(board.pieces(p, board.turn))
        score -= v * len(board.pieces(p, not board.turn))
    return score

class Node(object):
    def __init__(self, board : chess.Board):
        self.board = board
        self.parent = None
        self.children = []
        def compare(m):
            b = board.copy()
            b.push(m)
            return _evaluate_board(b)

        self.not_explored_legal_moves = sorted(board.legal_moves, key=compare, reverse=True)
        self.number_of_visits = 0
        self.average_score = 0.0

    def is_terminal(self):
        return len(self.not_explored_legal_moves) == 0 and len(self.children) == 0
    def is_expandable(self):
        return len(self.not_explored_legal_moves) != 0

    def uct_value(self):
        return self.average_score + 1.4142 * math.sqrt(math.log(self.parent.number_of_visits)/self.number_of_visits)

class MCST(object):
    def __init__(self, board):
        self.root = Node(board)
        self.player = board.turn
        self.opponent = not board.turn
    
    def _select(self, node : Node) -> Node:
        logging.debug("GOING DEEPER")
        if node.board == None:
            logging.debug("WTF shouldn't happen")
            return node
        logging.debug(node.board)
        if node.is_terminal() or node.is_expandable():
            return node
        else:
            best_child = max(node.children, key=lambda x: x[1].uct_value())
            return self._select(best_child[1])
    
    def _expand(self, node : Node) -> Node:
        logging.debug("Expansion")
        new_move = node.not_explored_legal_moves.pop()
        board = node.board.copy()
        board.push(new_move)
        new_child = Node(board)
        new_child.parent = node
        node.children.append((new_move, new_child))
        return new_child

    def _simulate(self, node : Node) -> float:
        #let's try a little hack
        return _evaluate_board(node.board)/200.0
        # board = node.board.copy()
        # while board.outcome(claim_draw=True) == None:
        #     move = random.choice(list(board.legal_moves))
        #     board.push(move)
        #     # logging.debug("Simulated position:")
        #     # logging.debug(board)
        # outcome = board.outcome(claim_draw=True)
        # if outcome == self.opponent:
        #     return 0.0
        # if outcome == self.player:
        #     return 1.0
        # return 0.5

    def _backpropagate(self, node : Node, game_stats : float):
        logging.debug("Backpropagating")
        node.average_score = (node.average_score * node.number_of_visits) + game_stats / (node.number_of_visits + 1)
        node.number_of_visits += 1
        if node.parent != None:
            self._backpropagate(node.parent, game_stats)
    
    def run_iteration(self):
        logging.debug("Start MCTS iteration")
        node = self._select(self.root)
        if not node.is_terminal():
            node = self._expand(node)
        logging.debug("Start simulation")
        game_stats = self._simulate(node)
        logging.debug("End simulation")
        self._backpropagate(node, game_stats)
    
    def get_best_move(self):
        best_move = max(self.root.children, key = lambda x: x[1].number_of_visits)
        return best_move[0]



