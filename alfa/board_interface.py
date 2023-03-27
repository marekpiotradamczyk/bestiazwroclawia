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
