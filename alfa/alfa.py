#!/usr/bin/python3

import time
import chess
import eval
import board_interface

p = 0
infinity = int(1e6)
CHECKMATE = infinity - 1
DRAW = 0

# Zaczątek dla alfa-bety

def search(board, depth):
    global p
    p += 1
    best_ev = -infinity
    if depth == 0:
        return eval.eval(board)
    moves = list(board_interface.get_moves(board))
    for m in moves:
        board = board_interface.make_move(board, m)
        ev = search(board, depth - 1)
        board = board_interface.reverse_move(board)
        best_ev = max(ev, best_ev)
    return -best_ev


def have_time(timestamp1):
    return time.time() - timestamp1 < 2


def make_move():
    timestamp1 = time.time()
    board = chess.Board()
    depth = 1
    while have_time(timestamp1):
        eval = search(board, depth)
        depth += 1
    print(eval, depth, p)



if __name__ == "__main__":
    make_move()
