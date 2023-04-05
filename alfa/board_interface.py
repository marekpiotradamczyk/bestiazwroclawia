#!/usr/bin/python3

import chess

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


def is_capture(board, m):
    return board.is_capture(m)


def has_captures(board):
    for m in list(board.legal_moves):
        if is_capture(board, m):
            return True
    return False


def filter_only_captures(board, moves):
    res = []
    for m in moves:
        if is_capture(board, m):
            res.append(m)
    return res
