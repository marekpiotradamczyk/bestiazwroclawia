#!/usr/bin/python3

import chess
import random

infinity = int(1e6)
CHECKMATE = infinity - 1
DRAW = 0


def piecesNum(board):
    pawns = len(board.pieces(1, True)) + len(board.pieces(1, False))
    knights = len(board.pieces(2, True)) + len(board.pieces(2, False))
    bishops = len(board.pieces(3, True)) + len(board.pieces(3, False))
    rooks = len(board.pieces(4, True)) + len(board.pieces(4, False))
    queens = len(board.pieces(5, True)) + len(board.pieces(5, False))
    return pawns + knights + bishops + rooks + queens


def piecesVal(board, side):
    # Ocena materiału
    pawn = len(board.pieces(1, side))
    knigt = len(board.pieces(2, side))
    bishop = len(board.pieces(3, side))
    rook = len(board.pieces(4, side))
    queen = len(board.pieces(5, side))
    return pawn + 3 * knigt + 3 * bishop + 5 * rook + 9 * queen


def doubledPawns(board, side):
    res = 0
    p = board.pieces(1, side)
    for i in p:
        for j in p:
            if i % 8 == j % 8:
                res += 0.5
    return res


def isolatedPawns(board, side):
    res = 0
    for i in board.pieces(1, side):
        isolated = True
        for j in board.pieces(1, side):
            if j == i - 1 or j == i + 1:
                isolated = False
        if isolated:
            res += 1
    return res


def pawnsGroups(board, side):
    res = 1
    tab = [False for i in range(8)]
    for i in board.pieces(1, side):
        col = i % 8
        tab[col] = True
    for i in range(1, 8):
        if not tab[i] and tab[i - 1]:
            res += 1
    return res


def pawnStructure(board, side):
    # Ocena struktury pionowej, na minus działają zdublowane lub izolowane pionki, i nadmierne grupy pionowe
    res = doubledPawns(board, side) + isolatedPawns(board, side) + (pawnsGroups(board, side) - 1)
    return res


def centerControl(board, move, side):
    # Naiwna ocena posiadania centrum
    res = 0
    if move < 24:
        for i in range(2, 6):
            for j in range(2, 6):
                square = 8 * i + j
                if i > 1 and i < 6 and j > 1 and j < 6:
                    if len(board.attackers(side, square)) > len(board.attackers(not side, square)) and square in board.pieces(1, side):
                        res += 1
                        if i > 2 and i < 5 and j > 2 and j < 5:
                            if square in board.pieces(1, side):
                                res += 3
    return res


def kingsSafety(board, side):
    res = board.legal_moves.count() - board.pseudo_legal_moves.count()
    for i in board.pieces(6, side):
        res += abs(4 - (i // 8)) / 4
        res += abs(4 - (i % 8)) / 4
    return res


# Kilka poniższych funkcji ma oceniać aktywność figur
def kingTropism(board, side):
    res = 0
    opp_king = list(board.pieces(6, not side))
    opp_king = opp_king[0]
    my_queens = list(board.pieces(5, side))
    for i in my_queens:
        res = abs((i // 8) - (opp_king // 8)) + abs((i % 8) - (opp_king % 8))
    return -res


def knightsPosition(board, side):
    knights = board.pieces(2, side)
    res = 0
    for k in knights:
        if k % 8 == 0 or k % 8 == 7:
            res -= 0.01
        elif k // 8 > 0 and k // 8 < 7:
            res += 0.01
    return res


def bishopsPosition(board, side):
    bishops = board.pieces(2, side)
    res = 0
    for b in bishops:
        res += 0.05 * len(list(board.attacks(b)))
    return res


def rooksPosition(board, side):
    rooks = board.pieces(2, side)
    res = 0
    for r in rooks:
        res += 0.05 * len(list(board.attacks(r)))
    return res


def eval(board):
    # Niezbyt dobra funkcja oceny pozycji, zostanie wyrzucona, gdy powstanie
    # sieć neuronowa dla oceny pozycji.
    # Wady takiej:
    # Jest bardzo wolna (też przez bibliotekę)
    # Nie jest zbyt dokładna
    a_mat = 1.6
    a_mob = 0.01
    a_cent = 0.4
    a_pstr = -0.05
    a_ksaf = 0.05
    a_ktro = 0.001
    a_check = 0.05
    side = board.turn
    m = board.halfmove_clock
    if board.is_checkmate():
        return -CHECKMATE
    if board.is_stalemate() or board.is_seventyfive_moves() or board.is_fivefold_repetition() or board.is_insufficient_material():
        return DRAW
    material = piecesVal(board, side) - piecesVal(board, not side)
    my_mobility = board.legal_moves.count()
    board.turn = not side
    opp_mobility = board.legal_moves.count()
    board.turn = side
    mobility = my_mobility - opp_mobility
    center_control = centerControl(board, m, side)
    pawn_structure = pawnStructure(board, side)
    king_safety = kingsSafety(board, side)
    king_tropism = kingTropism(board, side)
    checking = len(list(board.checkers()))
    res = a_mat * material + a_mob * mobility + a_cent * center_control
    res += a_pstr * pawn_structure + a_ksaf * king_safety + a_ktro * king_tropism
    res += a_check * checking + knightsPosition(board, side)
    res += bishopsPosition(board, side) + rooksPosition(board, side)
    return res
