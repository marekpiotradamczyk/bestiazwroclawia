import chess
import random
import math
import logging

_piece_values = {chess.PAWN : 1.0, chess.KNIGHT : 3.0, chess.BISHOP : 4.0, chess.ROOK : 5.0, chess.QUEEN : 9.0, chess.KING : 15.0}
_king_table = [[1.0, 2.0, 1.5, 1.0, 1.0, 1.0, 1.75, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 2.0, 1.5, 1.0, 1.0, 1.0, 1.75, 1.0]]
_queen_table = [[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]]
_rook_table = [[0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5]]
_bishop_table = [[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
[1.0, 1.25, 1.0, 1.0, 1.0, 1.0, 1.25, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
[1.0, 1.25, 1.0, 1.0, 1.0, 1.0, 1.25, 1.0],
[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5]]
_knight_table = [[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
[0.5, 0.5, 0.5, 0.75, 0.75, 0.5, 0.5, 0.5],
[0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
[0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
[0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
[0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
[0.5, 0.5, 0.5, 0.75, 0.75, 0.5, 0.5, 0.5],
[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5]]
_pawn_table = [[0, 0, 0, 0, 0, 0, 0, 0],
[3.0, 3.0, 3.0, 4.0, 4.0, 3.0, 3.0, 3.0],
[2.0, 2.0, 2.5, 3.5, 3.5, 2.5, 2.0, 2.0],
[1.75, 1.75, 2.0, 3.25, 3.25, 2.0, 1.75, 1.0],
[1.0, 1.0, 1.0, 3.0, 3.0, 1.0, 1.0, 1.0],
[1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0],
[1.0, 1.0, 1.5, 0.5, 0.5, 1.5, 1.0, 1.0],
[0, 0, 0, 0, 0, 0, 0, 0]]

_piece_tables = {chess.PAWN : _pawn_table, chess.KNIGHT : _knight_table, chess.BISHOP : _bishop_table, \
                 chess.ROOK : _rook_table, chess.QUEEN : _queen_table, chess.KING : _king_table}


def _evaluate_board(board : chess.Board) -> float:
    score = 0.0
    outcome = board.outcome(claim_draw=True)
    if outcome != None:
        if outcome.winner == None:
            return 0.0
        if outcome.winner == board.turn:
            return 10.0
        return -10.0
    
    def square_coord(s, turn):
        if turn == chess.WHITE:
            return 7-chess.square_rank(s), chess.square_file(s)
        else:
            return chess.square_rank(s), chess.square_file(s)
    for piece, table in _piece_tables.items():
        for s in board.pieces(piece, board.turn):
            r, c = square_coord(s, board.turn)
            score += _piece_values[piece] * table[r][c]
        for s in board.pieces(piece, not board.turn):
            r, c = square_coord(s, not board.turn)
            score -= _piece_values[piece] * table[r][c]
        # score += v * len(board.pieces(p, board.turn))
        # score -= v * len(board.pieces(p, not board.turn))
    return max(-30.0, min(30.0, score))

class Node(object):
    def __init__(self, board : chess.Board):
        self.board = board
        self.parent = None
        self.children = []
        def compare(m):
            b = board.copy()
            b.push(m)
            return _evaluate_board(b)

        # self.not_explored_legal_moves = None
        # if self.board.turn == chess.WHITE:
        self.not_explored_legal_moves = sorted(board.legal_moves, key=compare, reverse=False)
        # else:
            # self.not_explored_legal_moves = sorted(board.legal_moves, key=compare, reverse=False)
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
        # logging.debug(node.board)
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
        if node.board.turn == self.root.board.turn:
            return (_evaluate_board(node.board)+15.0)/30.0
        else:
            return (-_evaluate_board(node.board)+15.0)/30.0
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
        logging.debug("Root visits: "+str(self.root.number_of_visits))
        best_move = max(self.root.children, key = lambda x: x[1].number_of_visits)
        return best_move[0]



