#!/usr/bin/python3

import chess
import random

values = [0, 1, 3, 3, 5, 9, 0]
infinity = int(1e6)
CHECKMATE = infinity - 1


def piecesVal(board, side):
    # Ocena materiału
    res = 0
    for i in range(1, 6):
        pieces = len(board.pieces(i, side))
        res += values[i] * pieces
    return res

###
# Tablice do oceny pozycji/aktywności
###

tabs = {
    1: [
        [0,  0,  0,  0,  0,  0,  0,  0],
        [50, 50, 50, 50, 50, 50, 50, 50],
        [10, 10, 20, 30, 30, 20, 10, 10],
        [5,  5, 10, 25, 25, 10,  5,  5],
        [0,  0,  0, 20, 20,  0,  0,  0],
        [5, -5,-10,  0,  0,-10, -5,  5],
        [5, 10, 10,-20,-20, 10, 10,  5],
        [0,  0,  0,  0,  0,  0,  0,  0]
    ],
    2: [
    	[-50,-40,-30,-30,-30,-30,-40,-50],
        [-40,-20,  0,  0,  0,  0,-20,-40],
        [-30,  0, 10, 15, 15, 10,  0,-30],
        [-30,  5, 15, 20, 20, 15,  5,-30],
        [-30,  0, 15, 20, 20, 15,  0,-30],
        [-30,  5, 10, 15, 15, 10,  5,-30],
        [-40,-20,  0,  5,  5,  0,-20,-40],
        [-50,-40,-30,-30,-30,-30,-40,-50]
    ],
    3: [
        [-20,-10,-10,-10,-10,-10,-10,-20],
        [-10,  0,  0,  0,  0,  0,  0,-10],
        [-10,  0,  5, 10, 10,  5,  0,-10],
        [-10,  5,  5, 10, 10,  5,  5,-10],
        [-10,  0, 10, 10, 10, 10,  0,-10],
        [-10, 10, 10, 10, 10, 10, 10,-10],
        [-10,  5,  0,  0,  0,  0,  5,-10],
        [-20,-10,-10,-10,-10,-10,-10,-20]
    ],
    4: [
        [ 0,  0,  0,  0,  0,  0,  0,  0],
        [ 5, 10, 10, 10, 10, 10, 10,  5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [ 0,  0,  0,  5,  5,  0,  0,  0]
    ],
    5: [
    	[-20,-10,-10, -5, -5,-10,-10,-20],
        [-10,  0,  0,  0,  0,  0,  0,-10],
        [-10,  0,  5,  5,  5,  5,  0,-10],
        [-5,  0,  5,  5,  5,  5,  0, -5],
        [0,  0,  5,  5,  5,  5,  0, -5],
        [-10,  5,  5,  5,  5,  5,  0,-10],
        [-10,  0,  5,  0,  0,  0,  0,-10],
        [-20,-10,-10, -5, -5,-10,-10,-20]
    ],
    6: [
    	[-30,-40,-40,-50,-50,-40,-40,-30],
        [-30,-40,-40,-50,-50,-40,-40,-30],
        [-30,-40,-40,-50,-50,-40,-40,-30],
        [-30,-40,-40,-50,-50,-40,-40,-30],
        [-20,-30,-30,-40,-40,-30,-30,-20],
        [-10,-20,-20,-20,-20,-20,-20,-10],
        [ 20, 20,  0,  0,  0,  0, 20, 20],
        [ 20, 30, 10,  0,  0, 10, 30, 20]
    ],
    7: [
        [-50,-40,-30,-20,-20,-30,-40,-50],
        [-30,-20,-10,  0,  0,-10,-20,-30],
        [-30,-10, 20, 30, 30, 20,-10,-30],
        [-30,-10, 30, 40, 40, 30,-10,-30],
        [-30,-10, 30, 40, 40, 30,-10,-30],
        [-30,-10, 20, 30, 30, 20,-10,-30],
        [-30,-30,  0,  0,  0,  0,-30,-30],
        [-50,-30,-30,-30,-30,-30,-30,-50]
    ]
}

###
#
###

def endgame(board, piecesSum):
    return piecesSum < 22


def activity(board, piecesSum):
    res = 0
    for p in range(1, 6):
        pieceW = board.pieces(p, True)
        pieceB = board.pieces(p, False)
        for s in pieceW:
            x = s // 8
            y = s % 8
            res += tabs[p][x][y]
        for s in pieceB:
            x = 7 - (s // 8)
            y = s % 8
            res -= tabs[p][x][y]
    kingW = board.pieces(6, True).pop()
    kingB = board.pieces(6, False).pop()
    if not endgame(board, piecesSum):
        usedTab = 6
    else:
        usedTab = 7
    xW = kingW // 8
    yW = kingW % 8
    res += tabs[usedTab][xW][yW]
    xB = 7 - (kingB // 8)
    yB = kingB % 8
    res -= tabs[usedTab][xB][yB]
    return res


def eval(board):
    # return (random.random() - 0.5) * 60
    if board.is_checkmate():
        return CHECKMATE
    if board.is_fifty_moves() or board.is_fivefold_repetition() or board.is_stalemate() or board.is_insufficient_material():
        return 0
    whiteVal = piecesVal(board, True)
    blackVal = piecesVal(board, False)
    res = whiteVal - blackVal + (activity(board, whiteVal + blackVal) / 1000)
    # if not board.turn:
    #     res = -res
    return res
