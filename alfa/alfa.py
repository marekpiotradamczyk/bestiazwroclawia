#!/usr/bin/python3

import chess
import eval
import board_interface as brdInf
import zorba
import time
from queue import PriorityQueue

BestMove = ""
infinity = int(1e6)
HASHES = {}
Max_Depth = 125
killer_list = []
for i in range(Max_Depth):
    killer_list.append([])
ply_counter = 0
Return_now = 0
R = 2  # Parametr do Null Pruning


def Stop():
    global time_to_move
    time_to_move = 0


def PriorityList(pos, hash, depth, killer_moves):
    global HASHES

    moves = list(brdInf.get_moves(pos))
    move = ""
    if hash in HASHES:  # Najpierw sprawdzamy wierzchołek, który wcześniej uznaliśmy za najlepszy
        res = HASHES[hash]
        move = res[3]
    moves = brdInf.filter(pos, moves, move, killer_moves)
    return moves


def Qsearch(pos, alpha, beta):
    # Czy implementujemy tutaj TT?
    # Czy implementujemy tutaj PVS?
    evaluation = eval.eval(pos)
    if not brdInf.has_captures(pos) or evaluation > beta:  # Wszystkie bicia czy tylko na polu którym było wcześniej
        return evaluation
    BestSoFar = evaluation
    moves = brdInf.filter_only_captures(pos, list(brdInf.get_moves(pos)))
    while not moves.empty():
        move = moves.get()[2]
        brdInf.make_move(pos, move)
        val = -Qsearch(pos, -beta, -alpha)
        brdInf.reverse_move(pos)

        BestSoFar = max(val, BestSoFar)
        if BestSoFar > beta:
            return BestSoFar
        alpha = max(alpha, BestSoFar)
    return BestSoFar

# Alpha-Beta.
# Alfa - wynik dla strony której jest ruch - maksymalizujemy
# Beta - wynik dla przeciwnika - minimalizujemy
# StartDepth - Początkowa głębokość algorytmu
# BestMove - Najlepszy ruch znaleziony na początkowej głębokości
# BestTempMove - Najlepszy ruch znaleziony na aktualnej głębokości
# BestSoFar - Najlepszy wynik jaki znaleźliśmy


def AlphaBeta(pos, depth, alpha, beta, hash, StartDepth, time_to_move):
    global BestMove  # Przechowujemy najlepszy znaleziony do tej pory ruch (na poziomie)
    global killer_list
    global ply_counter
    global HASHES
    global Return_now
    global tStart


    # Transposition Table:
    # (TT.0) LINE = {depth, BestSoFar,NODE TYPE- "ALL"|"CUT"|"PV", BestTempMove}
    # (TT.1) Interesują nas tylko wyniki znalezione na conajmniej takiej samej głębokości
    # (TT.2) Jeżeli best so far(BSF) wierzchołka typu cut jest mniejszy od naszej bety, to nie musimy szukac dalej
    # (TT.3) Jeżeli jest większy od naszej alphy to analogicznie możemy tą alpha nadpisać,ale pozycję i tak trzeba przeszukać
    # (TT.4) Analogicznie dla wierzchołka typu "ALL"
    # (TT.5) Jeżeli wierzchołek jest typu "PV" to z zadaną dokładnością wyznazczyliśmy już BSF i możemy zwrócić
    # (TT.6) Znaleziony ruch juz jest za dobry dla przeciwnika.  - Typ CUT
    # (TT.7) Najlepszy ruchy byl gorszy niz najlepszy znaleziony wczesniej - Typ ALL
    # (TT.8) Znaleziony ruch poprawia nasza sytuacje - Typ PV

    if hash in HASHES:
        line = HASHES[hash]  # (TT.0)
        if line[0] >= depth:  # (TT.1)
            value = line[1]
            BestTempMove = line[3]
            node_type = line[2]
            if node_type == "CUT":
                if beta <= value:  # (TT.2)
                    if depth == StartDepth:
                        BestMove = BestTempMove
                    return value
                if alpha < value:  # (TT.3)
                    alpha = value
            if node_type == "ALL":  # (TT.4)
                if value <= alpha:
                    if depth == StartDepth:
                        BestMove = BestTempMove
                    return value
                if value < beta:
                    beta = value
            if node_type == "PV":  # (TT.5)
                if depth == StartDepth:
                    BestMove = BestTempMove
                return value

    # Alfa-Beta
    # (AB.1) Rekurencyjnie wchodzimy glebiej w pozycje, zmiana gracza ktory wykonuje ruch
    # (AB.2) Znaleziona pozycja jest lepsza niz najlepsza do tej pory
    # (AB.3) Warunek przerwania przeszukiwań, ustawiany z main
    # (AB.4) Przeszukaliśmy wszystkie mozliwe ruchy, zwracamy najlepszy znaleziony wynik
    #
    # Null Move Pruning (NMP) - Zakladamy ze istnieje lepszy ruch niz oddanie tury
    # (NMP.1) Sprawdzanie takiej hipotezy ma sens tylko wtedy, jeżeli w wyniku oddania tury otrzymamy legalna pozycje
    # (NMP.2) Jezeli oddanie tury bylo wieksze niz beta, to nie przegladamy żadnych dzieci
    #
    # PVS - Zakładamy że ruch jest gorszy niż poprzedni, sprawdzamy założenie
    # (PVS.1) Jezeli przeglądamy pierwszy wierzchołek, to musimy wykonac pelne przeszukiwanie
    # (PVS.2) Jeżeli hipoteza się potwierdziła, to przerywamy przeszukiwanie i zwracamy wynik
    # (PVS.3) Jeżeli hipoteza się nie potwierdziła to szukamy dokładniej
    # (PVS.4) Chyba że jesteśmy na głębokości 1, wtedy i tak nie sprawdzamy żadnych ruchów
    # (PVS.5) Znaleziona wartość z PVS może nam posłużyć za bardziej dokładną alphę
    #
    # Kliller Moves -
    # (KM.1) Jeżeli ruch nie był biciem a jednocześnie skutkował obcięciem, to chcemy go osobno zapamiętać
    #
    # HASHES
    # (H.1) Wyznaczamy hash dla danej pozycji i dla danego ruchu

    if depth <= 0:
        # Żeby wyłączyć QSearch należy zamienić zakomentowaną linijke
        # return eval.eval(pos)
        return Qsearch(pos, alpha, beta)
    BestTempMove = ""
    BestSoFar = -infinity

    if not brdInf.king_is_checked(pos):  # (NMP.1)
        newHash = zorba.hash(pos, hash, chess.Move.null(), pos.turn)
        val = -AlphaBeta(brdInf.afterpass(pos), depth - 1 - R, -beta, -alpha, newHash, StartDepth, time_to_move)
        pos = brdInf.reverse_move(pos)
        if val >= beta:  # (NMP.2)
            return val

    moves = PriorityList(pos, hash, depth, killer_list[ply_counter])
    firstIter = True  # (PVS.1)
    while not moves.empty():
        move = moves.get()[2]
        newHash = zorba.hash(pos, hash, move, pos.turn)  # (H.1)
        brdInf.make_move(pos, move)

        if not firstIter:
            val = -AlphaBeta(pos, depth - 1, -alpha, -alpha, newHash, StartDepth, time_to_move)
            if val < BestSoFar:  # (PVS.2)
                pos = brdInf.reverse_move(pos)
                continue

        if firstIter or (val < beta and depth > 2):  # (PVS.3) (PVS.4)
            if firstIter:
                val = alpha  # (PVS.5)
                firstIter = False
            val = -AlphaBeta(pos, depth - 1, -beta, -val, newHash, StartDepth, time_to_move)  # (AB.1)

        if val > BestSoFar:  # (AB.2)
            if depth == StartDepth:
                BestMove = move
            BestSoFar = val
            BestTempMove = move

        if time.time() - tStart > time_to_move:
            return BestSoFar

        pos = brdInf.reverse_move(pos)
        if BestSoFar >= beta:
            HASHES[hash] = (depth, BestSoFar, "CUT", BestTempMove)  # (TT.6)
            ply_number = ply_counter - depth
            if not brdInf.is_capture(pos, move) and move not in killer_list[ply_number]:  # (KM.1)
                killer_list[ply_number].insert(0, move)
                killer_list[ply_number] = killer_list[ply_number][1:]
            return BestSoFar
        alpha = max(alpha, BestSoFar)

    if BestSoFar < alpha:  # (TT.7)
        HASHES[hash] = (depth, BestSoFar, "ALL", BestTempMove)
    else:  # (TT.8)
        HASHES[hash] = (depth, BestSoFar,  "PV", BestTempMove)
    return BestSoFar  # (AB.4)


def Search(board, depth, time_to_move):
    global ply_counter
    global HASHES
    global Return_now
    global tStart
    # Iterative Deepening, domyslnie to powinno byc wolane przez main
    HASHES = {}
    posHash = zorba.hashInit(board)
    tStart = time.time()
    if depth is None:
        depth = 100
    if time_to_move is None:
        time_to_move = 10
    for i in range(1, depth + 1):
        ply_counter = i
        res = AlphaBeta(board, i, -infinity, infinity, posHash, i, time_to_move)
        # print(res, BestMove, i)
        # print(time.time() - tStart)
        if time.time() - tStart > time_to_move:
            return BestMove
        if res == 1000000:  # Znalezlismy wymuszonego mata, nie trzeba dalej szukac
            break
        # Warunek przerwania przeszukiwań, ustawiany z main
    return BestMove


# Przykladowe pozycje - mozna podmienic do testow
board = chess.Board()  # 0)
# board = chess.Board("2bk1b1r/p1pp1Qp1/2nq1n2/r1N3Bp/1pB1Pp1P/5NP1/PPPP4/R3K2R b KQ - 1 8")  # 1)
# board = chess.Board("r1bqkb1r/pp3ppp/2p1pn2/3p4/2nP4/P1N1PN1P/1PPB1PP1/R2QKB1R w KQkq - 2 8")  # 2)
# board = chess.Board("r2qkb1r/pp3ppp/2n5/3bp3/6Q1/7N/PPP2PPP/RNB1K2R b KQkq - 1 12")  # 3)
# board = chess.Board("r4rk1/3pb1pp/b2q1p2/R1p1N3/4PP2/2NP4/1PP3PP/3Q1RK1 w - - 0 17")  # 4)
# board = chess.Board("rnbqkbnr/ppp2ppp/8/3Pp3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 4")  # 5)
# board = chess.Board("4Q3/p4ppk/2N3qp/8/1p3n2/PP6/1P3PPK/8 b - - 0 27")  # 6)
board = chess.Board("4r2k/1p3rbp/2p3p1/p7/P2pB1nq/1P3n1N/6P1/B1Q1RR1K b - - 1 30")  # 7)
# board = chess.Board("6Q1/2pk1ppp/2p5/8/3P4/P1n1B3/1Rq2P1P/K7 b - - 8 29")
