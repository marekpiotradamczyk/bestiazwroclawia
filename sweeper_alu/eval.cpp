#include "eval.h"

#include <cassert>
#include <iostream>
#include <vector>

#include "position.h"
#include "types.h"

const int INF = 1000000;

int Eval(Position &position){
	int score = position.material[position.curr_side] - position.material[position.curr_side^1];
	score += position.curr_development[position.curr_side];
	score -= position.curr_development[position.curr_side^1];

	return score;
}
