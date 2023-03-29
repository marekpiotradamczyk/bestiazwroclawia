#include "search.h"

#include <algorithm>
#include <cassert>
#include <chrono>
#include <cstdlib>
#include <iostream>
#include <vector>

#include "eval.h"
#include "parser.h"
#include "position.h"
#include "types.h"

const int INF = 1000000;
const int MAXDEPTH = 100;
const int HASH_SIZE = 4*8388608;

struct{
	char depth;
	int score;
	unsigned int long_hash;
} hash_table[HASH_SIZE + 3];

std::vector<MoveType> debug_history;

bool cmp(std::pair<int, MoveType> a, std::pair<int, MoveType> b){
	return a.first < b.first;
}

MoveType Search::BestMove(Position &position){
	start_time = std::chrono::system_clock::now();
	target_time = std::chrono::duration<int, std::centi>(time_left/20);
	level_completed = true;
	MoveType best_move;
	std::vector<MoveType> moves = position.GeneratePseudoLegalMoves();
	unsigned int curr_hash = position.CurrentPositionHash(0);
	unsigned int long_hash = position.CurrentPositionHash(1);

	int prev_nodes = 1;
	for(int depth = 1; depth <= MAXDEPTH; depth++){
		int best = -INF-1;
		MoveType ret;
		eval_nodes = 0;
		cuttoffs = 0;
		transposition_usage = 0;
		std::vector<std::pair<int, MoveType>> candidates;
		for(const auto &move : moves){
			unsigned int new_hash = position.NewHash(curr_hash, position.curr_side, move, 0);
			unsigned int new_long_hash = position.NewHash(long_hash, position.curr_side, move, 1);
			MoveMetaDataType meta_data = position.StandardMetaData(move);
			position.MakeMove(move);
			unsigned int h = new_hash^position.AuxHash();
			if(hash_table[h].long_hash == new_long_hash){
				candidates.push_back(std::make_pair(hash_table[h].score, move));
			}
			else{
				candidates.push_back(std::make_pair(0, move));
			}
			position.UnmakeMove(move, meta_data);
		}

		stable_sort(candidates.begin(), candidates.end(), cmp);

		for(const auto &candidate : candidates){
			std::cout<< "#Try: " << MoveToString(candidate.second) << " " << candidate.first << "\n";
			MoveType move = candidate.second;
			MoveMetaDataType meta_data = position.StandardMetaData(move);
			unsigned int new_hash = position.NewHash(curr_hash, position.curr_side, move, 0);
			unsigned int new_long_hash = position.NewHash(long_hash, position.curr_side, move, 1);
			position.MakeMove(move);
			if(position.IsLegal()){
				debug_history.push_back(move);
				int cand = -NegaMax(position, depth-1, -INF, INF, new_hash, new_long_hash, move.dst);
				debug_history.pop_back();
				if(cand > best){
					best = cand;
					ret = move;
				}
			}
			position.UnmakeMove(move, meta_data);
		}

		if(level_completed){
			best_move = ret;
			auto now = std::chrono::system_clock::now();
			auto search_time = std::chrono::duration_cast<std::chrono::duration<int,std::centi>>(now-start_time).count();
			std::cout << depth << " " << best << " " << search_time << " " << eval_nodes << " " << MoveToString(best_move) << "\n";
			std::cout << "# cuttoffs: " << cuttoffs << "\n";
			std::cout << "# transposition_usage: " << transposition_usage << "\n";
			std::cout << "# effective branching factor: " << eval_nodes/prev_nodes << "\n";
			prev_nodes = std::max(1, eval_nodes);
			if(best == INF) break;
		}
		else {
			break;
		}
	}

	return best_move;
}

int Search::NegaMax(Position &position, int depth, int alpha, int beta, unsigned int curr_hash, unsigned int long_hash, int last_dst){
	assert(depth >= 0);

	unsigned int h = curr_hash^position.AuxHash();

	if(hash_table[h].long_hash == long_hash && hash_table[h].depth >= depth){
		transposition_usage++;
		return hash_table[h].score;
	}

	if(depth == 0){
		eval_nodes++;
		int eval = Quiescence(position, alpha, beta, last_dst);
		assert(h < HASH_SIZE);
		hash_table[h].depth = depth;
		hash_table[h].score = eval;
		hash_table[h].long_hash = long_hash;
		return eval;
	}

	int ret = -INF;
	std::vector<MoveType> moves = position.GeneratePseudoLegalMoves();

	int cnt = 0;
	for(const auto &move : moves){
		if(!level_completed) break;
		if(TimeIsOver()){
			level_completed = false;
			break;
		}

		MoveMetaDataType meta_data = position.StandardMetaData(move);
		unsigned int new_hash = position.NewHash(curr_hash, position.curr_side, move, 0);
		unsigned int new_long_hash = position.NewHash(long_hash, position.curr_side, move, 1);
		position.MakeMove(move);
		if(position.IsLegal()){
			cnt++;
			debug_history.push_back(move);
			int cand = -NegaMax(position, depth-1, -beta, -alpha, new_hash, new_long_hash, move.dst);
			debug_history.pop_back();
			ret = std::max(ret, cand);
			alpha = std::max(alpha, ret);
		}
		position.UnmakeMove(move, meta_data);
		if(alpha >= beta){
			cuttoffs++;
			return alpha;
		}
	}
	if(cnt == 0)
		return (position.IsMyKingAttaked() ? -INF : 0);

	hash_table[h].depth = depth;
	hash_table[h].score = ret;
	hash_table[h].long_hash = long_hash;
	return ret;
}

int Search::Quiescence(Position &position, int alpha, int beta, int last_dst){
	int stand_pat = Eval(position);
	if(stand_pat >= beta)
			return beta;
	if(alpha < stand_pat)
			alpha = stand_pat;

	std::vector<MoveType> moves = position.GeneratePseudoLegalCaptures();
	for(const auto &move : moves){
		if(!level_completed) break;
		if(move.dst != last_dst) continue;
		if(TimeIsOver()){
			level_completed = false;
			break;
		}

		MoveMetaDataType meta_data = position.StandardMetaData(move);
		position.MakeMove(move);
		if(position.IsLegal()){
			debug_history.push_back(move);
			int cand = -Quiescence(position, -beta, -alpha, move.dst);
			debug_history.pop_back();
			position.UnmakeMove(move, meta_data);

			if(cand >= beta){
				cuttoffs++;
				return beta;
			}
			if(cand > alpha){
				alpha = cand;
			}
		}
		else{
			position.UnmakeMove(move, meta_data);
		}
	}

	return alpha;
}

bool Search::TimeIsOver(){
	auto now = std::chrono::system_clock::now();
	return std::chrono::duration_cast<std::chrono::duration<int, std::centi>>(now-start_time) > target_time;
}
