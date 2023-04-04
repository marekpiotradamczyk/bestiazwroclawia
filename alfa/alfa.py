#!/usr/bin/python3

import chess
import eval
import board_interface
import zorba

infinity = int(1e6)
CHECKMATE = infinity - 1
DRAW = 0
HASHES = {}

# Wstępna wersja alpha bety. 
def AlphaBeta(pos, depth, alpha, beta, hash, StartDepth):
  global BestMove # Przechowujemy najlepszy znaleziony do tej pory ruch (na poziomie)
  if depth==0:
      return eval.eval(pos)
  if hash in HASHES: # odwiedziliśmy już tą pozycję wcześniej
    res=HASHES[hash]
    if res[0] >= depth: # Interesują nas tylko wyniki znalezione na conajmniej takiej samej głębokości
       v = res[1]
       if res[2] =="CUT": 
         if beta <= v: # Jeżeli best so far(BSF) wierzchołka typu cut jest mniejszy od naszej bety, to nie musimy szukac dalej
           return v
         if alpha < v: # Jeżeli jest większy od naszej alphy to analogicznie możemy tą alpha nadpisać, ale pozycję i tak trzeba przeszukać
           alpha = v
       if res[2] == "ALL": # Analogicznie dla wierzchołka typu "ALL"
         if v <= alpha:
           return v
         if v < beta:
           beta = v
       if res[2] == "PV": # Jeżeli wierzchołek jest typu "PV" to z zadaną dokładnością wyznazczyliśmy już BSF i możemy zwrócić
         return v
  BestTempMove="" 
  BestSoFar = -infinity
  moves = list(board_interface.get_moves(pos)) # TO DO: W tym miejscu kolejka priorytetowa
  if hash in HASHES: # Najpierw sprawdzamy wierdzchołek, który wcześniej uznaliśmy za najlepszy
    res=HASHES[hash]
    move=res[3]
    if move in moves:
      moves.remove(move)
      moves.insert(0,move)
  for move in moves:
      # Wyznaczamy hash dla danej pozycji i dla danego ruchu
      newHash = zorba.hash(pos, hash, move, pos.turn)
      pos = board_interface.make_move(pos, move)
      # Rekurencyjnie wchodzimy glebiej w pozycje, zmiana gracza ktory wykonuje ruch
      val = -AlphaBeta(pos, depth-1, -beta, -alpha, newHash, StartDepth)
      pos = board_interface.reverse_move(pos)
      # Znaleziona pozycja jest lepsza niz najlepsza do tej pory
      if val > BestSoFar:
        # Jezeli jestesmy na glebokosci poczatkowej, to jednoczesnie jest to stan globalny ktory osobno chcemy zaktualizowac
        if depth == StartDepth:
          BestMove=move
        BestSoFar=val
        BestTempMove=move
      if BestSoFar >= beta:
        # Znaleziony ruch juz jest za dobry dla przeciwnika
        # Nie interesuje nas czy przeciwnik zagra potencjalnie jeszcze lepszy ruch, przerywamy - Typ CUT
        HASHES[hash] = (depth, BestSoFar, "CUT", BestTempMove)
        return BestSoFar
      alpha = max(alpha, BestSoFar)
  
  # Przeszukaliśmy wszystkie mozliwe ruchy
  if BestSoFar < alpha: # Najlepszy ruchy byl gorszy niz najlepszy znaleziony wczesniej - Typ ALL
    HASHES[hash] = (depth, BestSoFar,  "ALL",BestTempMove)
  else:
    # Znaleziony ruch poprawia nasza sytuacje - Typ PV 
    HASHES[hash] = (depth, BestSoFar,  "PV", BestTempMove)
  return BestSoFar

def Search(board, depth):
  # Iterative Deepening, domyslnie to powinno byc wolane przez main
  HASHES = {}
  posHash = zorba.hashInit(board)
  for i in range(1,depth+1):
    print(AlphaBeta(board, i, -infinity, infinity, posHash, i))
