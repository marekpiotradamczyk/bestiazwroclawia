#!/usr/bin/python3

import time
import chess
import eval
import board_interface
import zorba

p = 0
infinity = int(1e6)
CHECKMATE = infinity - 1
DRAW = 0
HASHES = {}

# Zaczątek dla alfa-bety

def search(board, depth, hash):
    best_ev = -infinity
    if depth == 0:
        return eval.eval(board)
    moves = list(board_interface.get_moves(board))
    for m in moves:
        newHash = zorba.zorbaHash(board, hash, m, board.turn)
        board = board_interface.make_move(board, m)
        ev = search(board, depth - 1, newHash)
        board = board_interface.reverse_move(board)
        best_ev = max(ev, best_ev)
    HASHES[hash] = (depth, best_ev, board.turn)
    return -best_ev
