#!/usr/bin/ env python3

import random
import chess
import board_interface

zobristNumbers = {}


# Generator
def hashGen():
    for c in range(2):  # Dla każdego koloru
        for p in range(1, 7):  # Dla każdego rodzaju bierki
            for s in range(64):  # Dla każdego pola szachownicy
                number = random.randint(1, 2 ** 64)
                # Tworzymy liczbę dla hasha zobrista
                zobristNumbers[(p, c, s)] = number
    zobristNumbers["white_turn"] = random.randint(1, 2 ** 64)
    zobristNumbers["black_turn"] = random.randint(1, 2 ** 64)
    for color in [0, 1]:
        for length in ["long", "short"]:
            zobristNumbers[("castling", length, color)] = random.randint(1, 2 ** 64)
    for fromSquare in range(24, 33):
        toSquare1 = fromSquare - 9
        toSquare2 =  fromSquare - 7
        if fromSquare > 24:
            zobristNumbers[("en_passant", fromSquare, toSquare1)] = random.randint(1, 2 ** 64)
        if fromSquare < 31:
            zobristNumbers[("en_passant", fromSquare, toSquare2)] = random.randint(1, 2 ** 64)
    for fromSquare in range(32, 41):
        toSquare1 = fromSquare + 7
        toSquare2 = fromSquare + 9
        if fromSquare > 32:
            zobristNumbers[("en_passant", fromSquare, toSquare1)] = random.randint(1, 2 ** 64)
        if fromSquare < 39:
            zobristNumbers[("en_passant", fromSquare, toSquare2)] = random.randint(1, 2 ** 64)


# To musimy odpalać na początku gry, żeby wygenerować liczby do hashów oraz
# by policzyć hash pozycji początkowej
def hashInit(board):
    hashGen()
    hash = 0
    for square in board_interface.piece_map(board):
        piece = board_interface.piece_at(board, square)
        color = int(board_interface.color_at(board, square))
        hash ^= zobristNumbers[(piece, color, square)]
    hash ^= zobristNumbers["white_turn"]
    return hash


def hashEnPassRights(board, hash):
    for en_pass in board_interface.list_en_passant(board):
        fromSquare = board_interface.from_square(en_pass)
        toSquare = board_interface.to_square(en_pass)
        hash = hash ^ zobristNumbers[("en_passant", fromSquare, toSquare)]
    return hash


def hashCastlingRights(board, hash):
    color = int(board.turn)
    if board_interface.has_kingside_castling_rights(board):
        hash = hash ^ zobristNumbers[("castling", "short", color)]
    if board_interface.has_queenside_castling_rights(board):
        hash = hash ^ zobristNumbers[("castling", "long", color)]
    return hash


# To pewnie trzeba będzie napisać na nowo po dostaniu biblioteki rustowej
def hash(board, prevHash, move, turn):
    pieceColor = int(board.turn)
    # Nie chcemy, aby ten sam układ bierek, jednak dla różnych kolorów na ruchu
    # dawało tego samego hasha.
    hash = prevHash ^ zobristNumbers["white_turn"] ^ zobristNumbers["black_turn"]
    # Prawa do bić w przelocie i roszad
    hash = hashEnPassRights(board, hash)
    hash = hashCastlingRights(board, hash)
    board_interface.make_move(board, move)
    hash = hashEnPassRights(board, hash)
    hash = hashCastlingRights(board, hash)
    board_interface.reverse_move(board)

    #Jeżeli nasz ruch był Nullem (ponieważ oddajemy turę przeciwnikowi),
    # tu chcemy zakończyć liczenie hasha.
    if not bool(move):
        return hash

    # Ponieważ w szachach poza standardowymi ruchami są jeszcze promocje, roszady
    # i bicia w przelocie, musimy je wyifować, obawiam się
    if board_interface.promotion(move) is not None:
        fromSquare = board_interface.from_square(move)
        toSquare = board_interface.to_square(move)
        newPiece = board_interface.promotion(move)
        hash ^= zobristNumbers[(1, pieceColor, fromSquare)]
        hash ^= zobristNumbers[(newPiece, pieceColor, toSquare)]
    elif board_interface.is_castling(board, move):
        king = 6
        rook = 4
        # Roszady możemy, nieco brzydko, zahardkodować, ponieważ król i wieża
        # muszą być w pozycjach wyjściowych i iść na konkretne pozycje.
        fromSquare1 = 4 + pieceColor * 56
        if board_interface.is_kingside_castling(board, move):
            fromSquare2 = 7 + pieceColor * 56
            toSquare1 = 6 + pieceColor * 56
            toSquare2 = 5 + pieceColor * 56
        else:
            fromSquare2 = 0 + pieceColor * 56
            toSquare1 = 2 + pieceColor * 56
            toSquare2 = 4 + pieceColor * 56
        hash ^= zobristNumbers[(king, pieceColor, fromSquare1)]
        hash ^= zobristNumbers[(king, pieceColor, toSquare1)]
        hash ^= zobristNumbers[(rook, pieceColor, fromSquare2)]
        hash ^= zobristNumbers[(rook, pieceColor, toSquare2)]
    elif board_interface.is_en_passant(board, move):
        fromSquare = board_interface.from_square(move)
        toSquare = board_interface.to_square(move)
        if turn:
            beatenPieceSquare = toSquare - 8
        else:
            beatenPieceSquare = toSquare + 8
        hash ^= zobristNumbers[(1, pieceColor, fromSquare)]
        hash ^= zobristNumbers[(1, pieceColor, toSquare)]
        hash ^= zobristNumbers[(1, 1 - pieceColor, beatenPieceSquare)]
    else:
        fromSquare = board_interface.from_square(move)
        pieceType = board_interface.piece_at(board, fromSquare)
        toSquare = board_interface.to_square(move)
        beatenPieceType = board_interface.piece_at(board, toSquare)
        beatenPieceColor = 1 - pieceColor
        hash ^= zobristNumbers[(pieceType, pieceColor, fromSquare)]
        hash ^= zobristNumbers[(pieceType, pieceColor, toSquare)]
        if beatenPieceType is not None:
            hash ^= zobristNumbers[(beatenPieceType, beatenPieceColor, toSquare)]
    return hash
