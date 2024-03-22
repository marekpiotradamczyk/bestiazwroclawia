#ifndef PARSER_H
#define PARSER_H

#include<string>

#include "types.h"

MoveType StringToMove(std::string s);
std::string MoveToString(MoveType move);

#endif
