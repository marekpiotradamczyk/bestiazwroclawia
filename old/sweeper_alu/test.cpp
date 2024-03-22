#include<iostream>
#include<vector>

#include "position.h"
#include "types.h"

int main(){
  Position position;
  position.Init();

  position.TestPiecesInStartingPosition(WHITE);
  position.TestPiecesInStartingPosition(BLACK);
  position.TestFriendlyInStartingPosition(WHITE);
  position.TestFriendlyInStartingPosition(BLACK);
  position.TestKnightMovesInStartingPosition(WHITE);
  position.TestKnightMovesInStartingPosition(BLACK);
  position.TestPawnMovesInStartingPosition(WHITE);
  position.TestPawnMovesInStartingPosition(BLACK);

  position.TestBishopMovesWithOneOtherPiece(WHITE, A1, WHITE, B2, true, 0);
  position.TestBishopMovesWithOneOtherPiece(WHITE, A1, WHITE, B2, false, 0);
  position.TestBishopMovesWithOneOtherPiece(WHITE, A1, BLACK, B2, true, 1);
  position.TestBishopMovesWithOneOtherPiece(WHITE, A1, BLACK, B2, false, 0);
  position.TestBishopMovesWithOneOtherPiece(WHITE, A1, BLACK, B2, false, 0);
  position.TestBishopMovesWithOneOtherPiece(BLACK, D3, BLACK, B1, false, 10);
  position.TestBishopMovesWithOneOtherPiece(BLACK, D3, WHITE, B1, true, 1);
  position.TestBishopMovesWithOneOtherPiece(WHITE, D3, WHITE, A1, false, 11);
  position.TestBishopMovesWithOneOtherPiece(BLACK, D4, BLACK, F6, false, 10);

  position.TestKingMovesWithOneOtherPiece(WHITE, D4, WHITE, E4, false, 7);
  position.TestKingMovesWithOneOtherPiece(WHITE, D4, WHITE, E4, true, 0);
  position.TestKingMovesWithOneOtherPiece(WHITE, D4, BLACK, E4, true, 1);
  position.TestKingMovesWithOneOtherPiece(BLACK, A8, WHITE, E4, false, 3);
  position.TestKingMovesWithOneOtherPiece(BLACK, A8, WHITE, E4, true, 0);
  position.TestKingMovesWithOneOtherPiece(BLACK, A8, WHITE, B8, true, 1);
  position.TestKingMovesWithOneOtherPiece(BLACK, C8, WHITE, E4, false, 5);

  position.TestKnightMovesWithOneOtherPiece(WHITE, C4, WHITE, B6, false, 7);
  position.TestKnightMovesWithOneOtherPiece(WHITE, C4, BLACK, B6, true, 1);
  position.TestKnightMovesWithOneOtherPiece(WHITE, C4, BLACK, B6, false, 7);
  position.TestKnightMovesWithOneOtherPiece(WHITE, H1, WHITE, B6, false, 2);
  position.TestKnightMovesWithOneOtherPiece(WHITE, H7, WHITE, B6, false, 3);
  position.TestKnightMovesWithOneOtherPiece(BLACK, D1, WHITE, B6, false, 4);
  position.TestKnightMovesWithOneOtherPiece(BLACK, D1, WHITE, C3, true, 1);

  position.TestPawnMovesWithOneOtherPiece(WHITE, A2, WHITE, A4, false, 1);
  position.TestPawnMovesWithOneOtherPiece(WHITE, A2, WHITE, A4, true, 0);
  position.TestPawnMovesWithOneOtherPiece(WHITE, A2, BLACK, A3, false, 0);
  position.TestPawnMovesWithOneOtherPiece(WHITE, A2, BLACK, B3, true, 1);
  position.TestPawnMovesWithOneOtherPiece(BLACK, A3, WHITE, A4, false, 1);
  position.TestPawnMovesWithOneOtherPiece(WHITE, A2, WHITE, A8, false, 2);
  position.TestPawnMovesWithOneOtherPiece(BLACK, D4, WHITE, A4, false, 1);
  position.TestPawnMovesWithOneOtherPiece(BLACK, E4, WHITE, D3, true, 1);
  position.TestPawnMovesWithOneOtherPiece(BLACK, C7, WHITE, A4, false, 2);
  position.TestPawnMovesWithOneOtherPiece(BLACK, G7, WHITE, G5, false, 1);
  position.TestPawnMovesWithOneOtherPiece(BLACK, G7, WHITE, H6, true, 1);

  position.TestRookMovesWithOneOtherPiece(WHITE, A1, WHITE, A2, false, 7);
  position.TestRookMovesWithOneOtherPiece(WHITE, A1, WHITE, A2, true, 0);
  position.TestRookMovesWithOneOtherPiece(WHITE, A1, BLACK, A2, false, 7);
  position.TestRookMovesWithOneOtherPiece(WHITE, A1, BLACK, A2, true, 1);
  position.TestRookMovesWithOneOtherPiece(WHITE, D4, BLACK, G4, false, 12);
  position.TestRookMovesWithOneOtherPiece(BLACK, G4, BLACK, D4, false, 10);
  position.TestRookMovesWithOneOtherPiece(BLACK, G4, WHITE, D4, true, 1);
  position.TestRookMovesWithOneOtherPiece(BLACK, G4, BLACK, D4, true, 0);

  position.TestStartingPositionMovesGeneration(WHITE);
  position.TestStartingPositionMovesGeneration(BLACK);

  position.TestPerftFromStartingPosition(1, 20);
  position.TestPerftFromStartingPosition(2, 400);
  position.TestPerftFromStartingPosition(3, 8902);
  position.TestPerftFromStartingPosition(4, 197281);
  position.TestPerftFromStartingPosition(5, 4865609);
  position.TestPerftFromStartingPosition(6, 119060324);

  position.TestPromotionsSimply();
  position.TestPromotionsSimply2();

  position.TestPerftFromKiwipete(0, 1);
  position.TestPerftFromKiwipete(1, 48);
  position.TestPerftFromKiwipete(2, 2039);
  position.TestPerftFromKiwipete(3, 97862);
  position.TestPerftFromKiwipete(4, 4085603);
  position.TestPerftFromKiwipete(5, 193690690);

  position.TestPerftFromPosition3(0, 1);
  position.TestPerftFromPosition3(1, 14);
  position.TestPerftFromPosition3(2, 191);
  position.TestPerftFromPosition3(3, 2812);
  position.TestPerftFromPosition3(4, 43238);
  position.TestPerftFromPosition3(5, 674624);
  position.TestPerftFromPosition3(6, 11030083);
  position.TestPerftFromPosition3(7, 178633661);

  position.TestPerftFromStartingPositionInCrazyhouse(1, 20);
  position.TestPerftFromStartingPositionInCrazyhouse(2, 400);
  position.TestPerftFromStartingPositionInCrazyhouse(3, 8902);
  position.TestPerftFromStartingPositionInCrazyhouse(4, 197281);
  position.TestPerftFromStartingPositionInCrazyhouse(5, 4888832);
  position.TestPerftFromStartingPositionInCrazyhouse(6, 120812942);

  return 0;
}
