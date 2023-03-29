#include "parser.h"

#include <cassert>
#include <cctype>
#include <iostream>
#include <string>
#include <unordered_map>

#include "math.h"

std::unordered_map<char, PieceType> char_to_piecetype = {{'k', KING}, {'q', QUEEN},
  {'r', ROOK}, {'b', BISHOP}, {'n', KNIGHT}, {'p', PAWN}};

MoveType StringToMove(std::string s){
  assert(s.size() == 4 || s.size() == 5);
  if(s.size() == 4){
    if(s[1] == '@'){ // drop move
      int sq = SET_SQ(COL_INT(s[2]), ROW_INT(s[3]));
      return MoveType(sq, sq, char_to_piecetype.at(std::tolower(s[0])), DROP);
    }
    else{ // normal move
      return MoveType(SET_SQ(COL_INT(s[0]), ROW_INT(s[1])),
        SET_SQ(COL_INT(s[2]), ROW_INT(s[3])), UNSPECIFIED);
    }
  }
  if(s.size() == 5){ // promotion
    return MoveType(SET_SQ(COL_INT(s[0]), ROW_INT(s[1])),
      SET_SQ(COL_INT(s[2]), ROW_INT(s[3])), char_to_piecetype.at(s[4]));
  }
}

char piece[6] = {'K', 'Q', 'R', 'B', 'N', 'P'};

std::string MoveToString(MoveType move){
  std::string s;
  if(move.kind_of_a_move ==  DROP){
    s = ".@..";
    s[0] = piece[move.piece_dst];
    s[2] = COL_CHAR(move.dst);
    s[3] = ROW_CHAR(move.dst);
  }
  else if(move.kind_of_a_move == PROMOTION){
    s = ".....";
    s[0] = COL_CHAR(move.src);
    s[1] = ROW_CHAR(move.src);
    s[2] = COL_CHAR(move.dst);
    s[3] = ROW_CHAR(move.dst);
    s[4] = std::tolower(piece[move.piece_dst]);
  }
  else{ // normal move
    s = "....";
    s[0] = COL_CHAR(move.src);
    s[1] = ROW_CHAR(move.src);
    s[2] = COL_CHAR(move.dst);
    s[3] = ROW_CHAR(move.dst);
  }
  return s;
}
