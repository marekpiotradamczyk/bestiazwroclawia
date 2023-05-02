#!/usr/bin/python3

import chess
import eval
import board_interface
import zorba
import time
from queue import PriorityQueue

BestMove = ""
infinity = int(1e6)
CHECKMATE = infinity - 1
DRAW = 0
HASHES = {}
Max_Depth = 125
killer_list = []
for i in range(Max_Depth):
    killer_list.append([])
Branches_Checkd = 0  # Aby sprawdzić czy optymalizacje ucina głałęzie zliczamy ucięte gałęzie
ply_counter = 0
Return_now = 0
R = 2  # Parametr do Null Pruning


def Stop():
    global Return_now
    Return_now = 1


def PriorityList(pos, hash, depth):  # TO DO: W tym miejscu kolejka priorytetowa
    global killer_list
    global ply_counter
    global HASHES

    moves = list(board_interface.get_moves(pos))
    move = ""
    if hash in HASHES:  # Najpierw sprawdzamy wierzchołek, który wcześniej uznaliśmy za najlepszy
        res = HASHES[hash]
        move = res[3]
    moves = board_interface.filter(pos, moves, move, killer_list[ply_counter])
    # only_captures - Przeszukanie w miejscu
    return moves


# Wstępna wersja alpha bety.
def AlphaBeta(pos, depth, alpha, beta, hash, StartDepth):
    global BestMove  # Przechowujemy najlepszy znaleziony do tej pory ruch (na poziomie)
    global killer_list
    global Branches_Checkd
    global ply_counter
    global HASHES
    global Return_now

    if hash in HASHES:  # odwiedziliśmy już tą pozycję wcześniej
        res = HASHES[hash]
        if res[0] >= depth:  # Interesują nas tylko wyniki znalezione na conajmniej takiej samej głębokości
            v = res[1]
            BestTempMove = res[3]
            if res[2] == "CUT":
                if beta <= v:  # Jeżeli best so far(BSF) wierzchołka typu cut jest mniejszy od naszej bety, to nie musimy szukac dalej
                    Branches_Checkd += 1
                    if depth == StartDepth:
                        BestMove = BestTempMove
                    return v
                if alpha < v:  # Jeżeli jest większy od naszej alphy to analogicznie możemy tą alpha nadpisać, ale pozycję i tak trzeba przeszukać
                    alpha = v
            if res[2] == "ALL":  # Analogicznie dla wierzchołka typu "ALL"
                if v <= alpha:
                    Branches_Checkd += 1
                    if depth == StartDepth:
                        BestMove = BestTempMove
                    return v
                if v < beta:
                    beta = v
            if res[2] == "PV":  # Jeżeli wierzchołek jest typu "PV" to z zadaną dokładnością wyznazczyliśmy już BSF i możemy zwrócić
                Branches_Checkd += 1
                if depth == StartDepth:
                    BestMove = BestTempMove
                return v

    if depth <= 0:
        # Poniższą linijkę należy odkomentować a jeszcze poniższą wciąć, żeby
        # włączyć pogłębianie do końca wymian.
        # if not board_interface.has_captures(pos): #Wszystkie bicia czy tylko na polu którym było wcześniej
        return eval.eval(pos)
        # moves = board_interface.filter_only_captures(pos, list(board_interface.get_moves(pos)))
    else:
        moves = PriorityList(pos, hash, depth)
    BestTempMove = ""
    BestSoFar = -infinity
    # Null Move Pruning - Zakladamy ze istnieje lepszy ruch niz oddanie tury
    # TODO - Sprawdzenie czy oddanie tury powoduje legalną pozycję
    val = -AlphaBeta(board_interface.afterpass(pos), depth - 1 - R, -beta, -alpha, hash, StartDepth)
    pos = board_interface.reverse_move(pos)
    # Jezeli oddanie tury bylo wieksze niz beta, to nie przegladamy żadnych dzieci
    if val >= beta:
        return val
    firstIter = True  # Jezeli przeglądamy pierwszy wierzchołek, to musimy wykonac pelne przeszukiwanie
    while not moves.empty():
        # Wyznaczamy hash dla danej pozycji i dla danego ruchu
        move = moves.get()[2]
        newHash = zorba.hash(pos, hash, move, pos.turn)
        board_interface.make_move(pos, move)
        # PVS - Zakładamy że ruch jest gorszy niż poprzedni, sprawdzamy założenie
        if not firstIter:
            val = -AlphaBeta(pos, depth - 1, -alpha, -alpha, newHash, StartDepth)
            if val < BestSoFar:
                pos = board_interface.reverse_move(pos)
                continue
        # Jeżeli hipoteza się nie potwierdziła to szukamy dokładniej
        # Chyba że jesteśmy na głębokości 1, wtedy nie ma takiej potrzeby
        if firstIter or (val < beta and depth > 2):
            if firstIter:
                val = alpha
                firstIter = False
            # Rekurencyjnie wchodzimy glebiej w pozycje, zmiana gracza ktory wykonuje ruch
            val = -AlphaBeta(pos, depth - 1, -beta, -val, newHash, StartDepth)
        # Znaleziona pozycja jest lepsza niz najlepsza do tej pory
        if val > BestSoFar:
            # Jezeli jestesmy na glebokosci poczatkowej, to jednoczesnie jest to stan globalny ktory osobno chcemy zaktualizowac
            if depth == StartDepth:
                BestMove = move
            BestSoFar = val
            BestTempMove = move
        # Warunek przerwania przeszukiwań, ustawiany z main
        if Return_now:
            return BestSoFar
        pos = board_interface.reverse_move(pos)
        if BestSoFar >= beta:
            if not board_interface.is_capture(pos, move) and move not in killer_list:
                killer_list[ply_counter - depth].insert(0, move)
                killer_list[ply_counter - depth] = killer_list[ply_counter - depth][1:]
            # Znaleziony ruch juz jest za dobry dla przeciwnika
            # Nie interesuje nas czy przeciwnik zagra potencjalnie jeszcze lepszy ruch, przerywamy - Typ CUT
            HASHES[hash] = (depth, BestSoFar, "CUT", BestTempMove)
            Branches_Checkd += 1
            return BestSoFar
        alpha = max(alpha, BestSoFar)
    # Przeszukaliśmy wszystkie mozliwe ruchy
    if BestSoFar < alpha:  # Najlepszy ruchy byl gorszy niz najlepszy znaleziony wczesniej - Typ ALL
        HASHES[hash] = (depth, BestSoFar, "ALL", BestTempMove)
    else:
        # Znaleziony ruch poprawia nasza sytuacje - Typ PV
        HASHES[hash] = (depth, BestSoFar,  "PV", BestTempMove)
    return BestSoFar


def Search(board, depth):
    global ply_counter
    global HASHES
    global Return_now
    # Iterative Deepening, domyslnie to powinno byc wolane przez main
    HASHES = {}
    posHash = zorba.hashInit(board)
    ts = time.time()
    for i in range(1, depth + 1):
        ply_counter = i
        print(AlphaBeta(board, i, -infinity, infinity, posHash, i), BestMove, i)
        print(time.time() - ts)
        # Warunek przerwania przeszukiwań, ustawiany z main
        if Return_now:
            break


# Przykladowe pozycje - mozna podmienic do testow
board = chess.Board()  # 0)
# board = chess.Board("2bk1b1r/p1pp1Qp1/2nq1n2/r1N3Bp/1pB1Pp1P/5NP1/PPPP4/R3K2R b KQ - 1 8")  # 1)
# board = chess.Board("r1bqkb1r/pp3ppp/2p1pn2/3p4/2nP4/P1N1PN1P/1PPB1PP1/R2QKB1R w KQkq - 2 8")  # 2)
# board = chess.Board("r2qkb1r/pp3ppp/2n5/3bp3/6Q1/7N/PPP2PPP/RNB1K2R b KQkq - 1 12")  # 3)
# board = chess.Board("r4rk1/3pb1pp/b2q1p2/R1p1N3/4PP2/2NP4/1PP3PP/3Q1RK1 w - - 0 17")  # 4)
# board = chess.Board("rnbqkbnr/ppp2ppp/8/3Pp3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 4")  # 5)
Search(board, 8)
# print(Branches_Checkd)
