#ifndef TYPES_H
#define TYPES_H

typedef unsigned long long int U64;
typedef unsigned long long int Bitboard;

enum PieceType{
	KING,
  QUEEN,
  ROOK,
  BISHOP,
  KNIGHT,
  PAWN,
  EMPTY,
	UNSPECIFIED
};

enum PieceColor{
	WHITE = 0,
	BLACK = 1
};

enum CastleRight{
  CASTLE_WK = 1,
  CASTLE_WQ = 2,
  CASTLE_BK = 4,
  CASTLE_BQ = 8
};

enum SquareCoor{
  A1, B1, C1, D1, E1, F1, G1, H1,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A8, B8, C8, D8, E8, F8, G8, H8
};

enum KindOfAMove{
	NORMAL,
	PROMOTION,
	DROP
};

struct MoveType{
	int src, dst;
	PieceType piece_dst;
	KindOfAMove kind_of_a_move;

	MoveType() {};

	MoveType(int x, int y, PieceType z){
		src = x; dst = y;
		piece_dst = z;
		kind_of_a_move = NORMAL;
	}

	MoveType(int x, int y, PieceType z, KindOfAMove a){
		src = x; dst = y;
		piece_dst = z;
		kind_of_a_move = a;
	}
};

struct MoveMetaDataType{
	PieceType what_was_there;
	int en_passant;
	Bitboard castle_rights[2];
	PieceType what_piece_was_moving;

	MoveMetaDataType(PieceType x, int y, Bitboard a, Bitboard b, PieceType c){
		what_was_there = x;
		en_passant = y;
		castle_rights[WHITE] = a;
		castle_rights[BLACK] = b;
		what_piece_was_moving = c;
	}
};

#endif // TYPES_H
