#!/bin/bash

debug() {
	var=$1
	echo ${var#*/}
}


play_vs_stockfish() {
	full_name=$1
	net=${full_name#*/}
	echo "Testing network: ${net} vs Stockfish 2500"

	python3 play.py --engine1 "testengines/morphebot_nn_12 $net" name=morphebot_nn --games 50 --start_elo 2500 --end_elo 2500 --time_control $2+0 --concurrency 4 --errors
	
	save_cleanup $net stockfish_$2
}

play_vs_morphe_blank() {
	full_name=$1
	net=${full_name#*/}
	echo "Testing network: ${net} vs morphe_blank"

	python3 play.py --engine1 "testengines/morphebot_nn_12 $net" name=morphebot_nn --engine2 "testengines/morphebot_blank $net" name=morphebot_blank --games 50 --start_elo 2500 --end_elo 2500 --time_control $2+0 --concurrency 4 --errors
	
	save_cleanup $net morphebot_blank_$2
}

play_vs_morphe_random() {
	full_name=$1
	net=${full_name#*/}
	echo "Testing network: ${net} vs morphe_random"
	
	python3 play.py --engine1 "testengines/morphebot_nn_12 $net/" name=morphebot_nn --engine2 "testengines/morphebot_random $net/" name=morphebot_random --games 50 --start_elo 2500 --end_elo 2500 --time_control $2+0 --concurrency 4 --errors
	
	save_cleanup $net morphebot_random_$2
}

save_cleanup() {
	(cd results && ./make_logs.sh new_results/$2/$1/ morphebot_nn morphebot_random)
	rm *.log games.pgn
	echo "Results saved to $1"
}

export -f debug
export -f play_vs_stockfish
export -f play_vs_morphe_blank
export -f play_vs_morphe_random
export -f save_cleanup

#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_stockfish "{}" 1' \;
#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_stockfish "{}" 10' \;
#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_stockfish "{}" 60' \;

#echo "Running test on tc 1+0"
#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_morphe_blank "{}" 1' \;
#echo "Running test on tc 10+0"
#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_morphe_blank "{}" 10' \;
#echo "Running test on tc 60+0"
#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_morphe_blank "{}" 60' \;

#echo "Running test on tc 1+0"
#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_morphe_random "{}" 1' \;
echo "Running test on tc 10+0"
find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_morphe_random "{}" 10' \;
#echo "Running test on tc 60+0"
#find testengines/weights -mindepth 2 -maxdepth 2 -exec bash -c 'play_vs_morphe_random "{}" 60' \;


