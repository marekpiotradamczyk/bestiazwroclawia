#!/usr/bin/python3

import chess
import eval
import board_interface
import zorba
#import time
from queue import PriorityQueue

infinity = int(1e6)
CHECKMATE = infinity - 1
DRAW = 0
HASHES = {}
Max_Depth = 125
killer_list = []
for i in range(Max_Depth):
    killer_list.append([])
Branches_Checkd = 0  # Aby sprawdzić czy optymalizacje ucina głałęzie zliczamy uciętegałęzie
ply_counter = 0


def PriorityList(pos, hash,depth): # TO DO: W tym miejscu kolejka priorytetowa
    global killer_list
    global ply_counter_counter
    moves = list(board_interface.get_moves(pos))
    move = ""
    if hash in HASHES:  # Najpierw sprawdzamy wierzchołek, który wcześniej uznaliśmy za najlepszy
        res = HASHES[hash]
        move = res[3]
    moves = board_interface.filter(pos, moves, move, killer_list) #only_captures - Przeszukanie w miejscu
    # killer moves
    #for move in killer_list[ply_counter-depth]:
     #  moves.remove(move)
      # moves.insert(0, move)
        
    # 1 HashMove
    # 2 KillerMoves
    # 3 Captures
    # 4 Rest
    return moves

# Wstępna wersja alpha bety.
def AlphaBeta(pos, depth, alpha, beta, hash, StartDepth):
    global BestMove  # Przechowujemy najlepszy znaleziony do tej pory ruch (na poziomie)
    global killer_list
    global Branches_Checkd
    global ply_counter
    if hash in HASHES:  # odwiedziliśmy już tą pozycję wcześniej
        res = HASHES[hash]
        if res[0] >= depth:  # Interesują nas tylko wyniki znalezione na conajmniej takiej samej głębokości
            v = res[1]
            if res[2] == "CUT":
                if beta <= v:  # Jeżeli best so far(BSF) wierzchołka typu cut jest mniejszy od naszej bety, to nie musimy szukac dalej
                    Branches_Checkd += 1
                    return v
                if alpha < v:  # Jeżeli jest większy od naszej alphy to analogicznie możemy tą alpha nadpisać, ale pozycję i tak trzeba przeszukać
                    alpha = v
            if res[2] == "ALL":  # Analogicznie dla wierzchołka typu "ALL"
                if v <= alpha:
                    Branches_Checkd += 1
                    return v
                if v < beta:
                    beta = v
            if res[2] == "PV":  # Jeżeli wierzchołek jest typu "PV" to z zadaną dokładnością wyznazczyliśmy już BSF i możemy zwrócić
                Branches_Checkd += 1
                return v

    if depth <= 0:
        # Poniższą linijkę należy odkomentować a jeszcze poniższą wciąć, żeby
        # włączyć pogłębianie do końca wymian.
        #if not board_interface.has_captures(pos): #Wszystkie bicia czy tylko na polu którym było wcześniej
        return eval.eval(pos)
        #moves = board_interface.filter_only_captures(pos, list(board_interface.get_moves(pos)))
    else:
        moves = PriorityList(pos,hash, depth)
    BestTempMove = ""
    BestSoFar = -infinity
    while not moves.empty():
        # Wyznaczamy hash dla danej pozycji i dla danego ruchu
        move = moves.get()[2]
        newHash = zorba.hash(pos, hash, move, pos.turn)
        pos = board_interface.make_move(pos, move)
        # Rekurencyjnie wchodzimy glebiej w pozycje, zmiana gracza ktory wykonuje ruch
        val = -AlphaBeta(pos, depth-1, -beta, -alpha, newHash, StartDepth)
        pos = board_interface.reverse_move(pos)
        # Znaleziona pozycja jest lepsza niz najlepsza do tej pory
        if val > BestSoFar:
            # Jezeli jestesmy na glebokosci poczatkowej, to jednoczesnie jest to stan globalny ktory osobno chcemy zaktualizowac
            if depth == StartDepth:
                BestMove = move
            BestSoFar = val
            BestTempMove = move
        if BestSoFar >= beta:
            if not board_interface.is_capture(pos, move) and move not in killer_list:
                killer_list[ply_counter-depth].insert(0, move)
                killer_list[ply_counter-depth] = killer_list[ply_counter-depth][1:]
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
    # Iterative Deepening, domyslnie to powinno byc wolane przez main
    HASHES = {}
    posHash = zorba.hashInit(board)
    #ts=time.time()
    for i in range(1, depth + 1):
        ply_counter = i
        print(AlphaBeta(board, i, -infinity, infinity, posHash, i), BestMove)
        #print(time.time()-ts)

board = chess.Board()
Search(board, 5)
#print(Branches_Checkd)
