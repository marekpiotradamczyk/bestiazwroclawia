#ifndef POSITION_H
#define POSITION_H

#include<unordered_map>
#include<vector>

#include "types.h"

class Position{
public:
	Position();

	unsigned int AuxHash();
	unsigned int CurrentPositionHash(int ind);
	void Debug();
	std::vector<MoveType> GeneratePseudoLegalMoves();
	std::vector<MoveType> GeneratePseudoLegalCaptures();
	int GetDevelopment(int side, PieceType piece);
	void Init();
	bool IsLegal();
	bool IsMyKingAttaked();
	void MakeMove(MoveType move);
	void NewGame();
	unsigned int NewHash(unsigned int old_hash, int side, MoveType move, int ind);
	int PieceCnt(int side, int piece_type);
	MoveMetaDataType StandardMetaData(MoveType move);
	void UnmakeMove(MoveType move, MoveMetaDataType data);

	int curr_side;
	bool crazyhouse;
	int stash[2][6];
	Bitboard friendly[2];
	int material[2];
	int curr_development[2];

	//Tests
	void TestKnightMovesInStartingPosition(int side);
	void TestPawnMovesInStartingPosition(int side);
	void TestPiecesInStartingPosition(int side);
	void TestFriendlyInStartingPosition(int side);
	void TestBishopMovesWithOneOtherPiece(int side, int bishop_pos,
		int other_side, int other_pos, bool captures, int expected_moves_cnt);
	void TestKingMovesWithOneOtherPiece(int side, int king_pos,
		int other_side, int other_pos, bool captures, int expected_moves_cnt);
	void TestKnightMovesWithOneOtherPiece(int side, int knight_pos,
		int other_side, int other_pos, bool captures, int expected_moves_cnt);
	void TestPawnMovesWithOneOtherPiece(int side, int pawn_pos,
		int other_side, int other_pos, bool captures, int expected_moves_cnt);
	void TestRookMovesWithOneOtherPiece(int side, int rook_pos,
		int other_side, int other_pos, bool captures, int expected_moves_cnt);
	void TestStartingPositionMovesGeneration(int side);
	void TestPerftFromStartingPosition(int depth, int expected_moves_cnt);
	void TestPerftFromKiwipete(int depth, int expected_moves_cnt);
	void TestPromotionsSimply();
	void TestPromotionsSimply2();
	void TestPerftFromPosition3(int depth, int expected_moves_cnt);
	void TestPerftFromStartingPositionInCrazyhouse(int depth, int expected_moves_cnt);
private:
	void ClearBoard();
	inline void EraseFromBoard(int side, int sq);
	void GenerateCastles(int side, std::vector<MoveType> &move_list);
	void GenerateBishopMoves(int side, bool captures, std::vector<MoveType> &move_list);
	void GenerateDrops(int side, std::vector<MoveType> &move_list);
	void GenerateKingMoves(int side, bool captures, std::vector<MoveType> &move_list);
	void GenerateKnightMoves(int side, bool captures, std::vector<MoveType> &move_list);
	void GeneratePawnMoves(int side, std::vector<MoveType> &move_list);
	void GeneratePawnCaptures(int side, Bitboard en_passant, std::vector<MoveType> &move_list);
	std::vector<MoveType> GeneratePseudoLegalMoves(int side);
	void GeneratePseudoLegalCaptures(int side, std::vector<MoveType> &move_list);
	void GeneratePseudoLegalNonCaptures(int side, std::vector<MoveType> &move_list);
	void GenerateQueenMoves(int side, bool captures, std::vector<MoveType> &move_list);
	void GenerateRookMoves(int side, bool captures, std::vector<MoveType> &move_list);
	void GenerateNonSliding(Bitboard occupancy, int side, Bitboard moves[], bool captures, std::vector<MoveType> &move_list);
	void GenerateSliding(
	  Bitboard occupancy,
	  int side,
	  Bitboard mask[],
		U64 moves[][4096],
		U64 magic[],
	  bool captures,
	 	std::vector<MoveType> &move_list);
	bool IsAttacked(int sq, int by_side);
	void Kiwipete();
	void MakeMove(int side, MoveType move);
	U64 Perft(int side, int depth, int &captures_no);
	void Position3();
	void PrecomputeBishopMoves();
	void PrecomputeDevelopment(PieceType piece, int table[]);
	void PrecomputeRookMoves();
	void Preprocessing();
	void PrintBitboard(Bitboard bitboard);
	void PrintChessBoard(PieceType chessboard[]);
	Bitboard ProcessBishopSubset(int sq, Bitboard subset);
	Bitboard ProcessRookSubset(int sq, Bitboard subset);
	void UnmakeMove(int side, MoveType move, MoveMetaDataType data);
	inline void SetPieceOnASquare(int sq, int side, PieceType piece_type);
	void SimplyPerformMove(int side, MoveType move);
	void StartingPosition();

	Bitboard pieces[2][6];
	Bitboard castle_rights[2];
	PieceType piece_on_square[64];
	int en_passant;

	unsigned int zobrist_piece[2][2][7][64];
	unsigned int zobrist_stash[2][2][7][21];
	unsigned int zobrist_en_passant[65];
	unsigned int zobrist_castle[2][256];
	unsigned int zobrist_side[2];
};

#endif // POSITION_H
