#!/usr/bin/ env python3

import random
import chess

zobristNumbers = {}

# Generator
def hashGen():
    for c in range(2): # Dla każdego koloru
        for p in range(1, 7): # Dla każdego rodzaju bierki
            for s in range(64): # Dla każdego pola szachownicy
                number = random.randint(1, 2**64)
                zobristNumbers[(p, c, s)] = number # Tworzymy liczbę dla hasha zobrista
    zobristNumbers[0] = random.randint(1, 2**64)
    zobristNumbers[1] = random.randint(1, 2**64)


# To musimy odpalać na początku gry, żeby wygenerować liczby do hashów oraz
# by policzyć hash pozycji początkowej
def hashInit(board):
    hashGen()
    hash = 0
    for square in board.piece_map():
        piece = board.piece_type_at(square)
        color = [1, 0][board.color_at(square)]
        hash ^= zobristNumbers[(piece, color, square)]
    hash ^= zobristNumbers[0]
    return hash


# To pewnie trzeba będzie napisać na nowo po dostaniu biblioteki rustowej
def hash(board, prevHash, move, turn):
    pieceColor = [1, 0][board.turn]
    # Nie chcemy, aby ten sam układ bierek, jednak dla różnych kolorów na ruchu
    # dawało tego samego hasha.
    # Można by się zastanowić, czy prawa do roszad lub bić w przelocie też
    # powinny to zmieniać (a być może powinny)
    hash = prevHash ^ zobristNumbers[0] ^ zobristNumbers[1]
    # Ponieważ w szachach poza standardowymi ruchami są jeszcze roszady
    # i bicia w przelocie, musimy je wyifować, obawiam się
    if move.promotion is not None:
        fromSquare = move.from_square
        toSquare = move.to_square
        newPiece = move.promotion
        hash ^= zobristNumbers[(1, pieceColor, fromSquare)]
        hash ^= zobristNumbers[(newPiece, pieceColor, toSquare)]
    elif board.is_castling(move):
        king = 6
        rook = 4
        # Roszady możemy, nieco brzydko, zahardkodować, ponieważ król i wieża
        # muszą być w pozycjach wyjściowych i iść na konkretne pozycje.
        fromSquare1 = 4 + pieceColor * 56
        if board.is_kingside_castling(move):
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
    elif board.is_en_passant(move):
        fromSquare = move.from_square
        toSquare = move.to_square
        if turn:
            beatenPieceSquare = toSquare - 8
        else:
            beatenPieceSquare = toSquare + 8
        hash ^= zobristNumbers[(1, pieceColor, fromSquare)]
        hash ^= zobristNumbers[(1, pieceColor, toSquare)]
        hash ^= zobristNumbers[(1, 1 - pieceColor, beatenPieceSquare)]
    else:
        fromSquare = move.from_square
        pieceType = board.piece_type_at(fromSquare)
        toSquare = move.to_square
        beatenPieceType = board.piece_type_at(toSquare)
        beatenPieceColor = 1 - pieceColor
        hash ^= zobristNumbers[(pieceType, pieceColor, fromSquare)]
        hash ^= zobristNumbers[(pieceType, pieceColor, toSquare)]
        if beatenPieceType is not None:
            hash ^= zobristNumbers[(beatenPieceType, beatenPieceColor, toSquare)]
    return hash
