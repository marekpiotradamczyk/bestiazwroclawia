#!/usr/bin/python3

import chess
from queue import PriorityQueue
# Moduł dla przeszukiwania drzewa gry do rozmawiania z biblioteką, na razie python-chess
# Jak już będziemy mieli tę rustową, to będziemy tu z nią gadać i te funkcje wywoływać w przeszukiwaniu


def get_moves(board):
    # Funkcja dostająca od biblioteki listę ruchów możliwych do wykonania w pozycji
    return list(board.legal_moves)


def make_move(board, move):
    # Funkcja puszczająca ruch na szachownicę
    board.push(move)
    return board


def reverse_move(board):
    # Funkcja cofająca ostatni zrobiony ruch (taką możliwość daje nam biblioteka),
    # jeśli ta w ruście nie da takiej łatwej możliwości, trzeba będzie bardziej kombinować
    board.pop()
    return board


def piece_map(board):
    # Funkcja zwracająca mapę (mogłaby być lista) pól na których sa bierki
    return board.piece_map()


def piece_at(board, square):
    return board.piece_type_at(square)


def color_at(board, square):
    return board.color_at(square)


def promotion(move):
    return move.promotion


def from_square(move):
    return move.from_square


def to_square(move):
    return move.to_square


def is_castling(board, move):
    return board.is_castling(move)


def is_kingside_castling(board, move):
    return board.is_kingside_castling(move)


def is_en_passant(board, move):
    return board.is_en_passant(move)


def is_capture(board, move):
    return board.is_capture(move)


def has_captures(board):
    for m in list(board.legal_moves):
        if is_capture(board, m):
            return True
    return False


def filter_only_not_captures(board, moves):
    res = []
    for m in moves:
        if not is_capture(board, m):
            res.append(m)
    return res


def has_queenside_castling_rights(board):
    board.has_queenside_castling_rights(board.turn)


def has_kingside_castling_rights(board):
    board.has_kingside_castling_rights(board.turn)


def list_en_passant(board):
    res = []
    for m in get_moves(board):
        if is_en_passant(board, m):
            res.append(m)
    return res


def king_is_checked(board):
    return board.is_check()


def afterpass(board):
    make_move(board, chess.Move.null())
    return board


# MVV_LVA[victim][attacker]
MVV_LVA = [
    [0, 0, 0, 0, 0, 0, 0],              # victim K, attacker K, Q, R, B, N, P, None
    [-50, -51, -52, -53, -54, -55, 0],  # victim Q, attacker K, Q, R, B, N, P, None
    [-40, -41, -42, -43, -44, -45, 0],  # victim R, attacker K, Q, R, B, N, P, None
    [-30, -31, -32, -33, -34, -35, 0],  # victim B, attacker K, Q, R, B, N, P, None
    [-20, -21, -22, -23, -24, -25, 0],  # victim N, attacker K, Q, R, B, N, P, None
    [-10, -11, -12, -13, -14, -15, 0],  # victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],          # victim None, attacker K, Q, R, B, N, P, None
]
# Piece_Position= ["K","Q","R","B","N","P",None] # Pozycja w tabeli MVV_LVA


def filter_only_captures(board, moves):
    Q = PriorityQueue()
    counter = 0
    for move in moves:
        if is_en_passant(board, move):
            victim = 6 - 1
            attacker = 6 - 1
            Q.put((-MVV_LVA[victim][attacker], counter, move))
        elif is_capture(board, move):
            pieceAt = piece_at(board, to_square(move))
            if pieceAt == 6:
                continue
            victim = 6 - pieceAt  # Znajdujemy typ bitej bierki
            attacker = 6 - piece_at(board, from_square(move))
            Q.put((MVV_LVA[victim][attacker], counter, move))
        counter += 1
    return Q


def filter(board, moves, hashmove, killerlist):
    # 1 HashMove
    # 2 KillerMoves
    # 3 Captures
    # 4 Rest
    counter = 0
    Q = PriorityQueue()
    for move in moves:
        if move == hashmove:
            Q.put((-12345, counter, move))
        elif move in killerlist:
            Q.put((-12344, counter, move))
        elif is_en_passant(board, move):
            victim = 6 - 1
            attacker = 6 - 1
            Q.put((-MVV_LVA[victim][attacker], counter, move))
        elif is_capture(board, move):
            pieceAt = piece_at(board, to_square(move))
            victim = 6 - pieceAt  # Znajdujemy typ bitej bierki
            attacker = 6 - piece_at(board, from_square(move))
            Q.put((MVV_LVA[victim][attacker], counter, move))
        else:
            Q.put((0, counter, move))
        counter += 1
    return Q
