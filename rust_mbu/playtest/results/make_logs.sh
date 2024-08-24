#!/bin/bash
mkdir -p $1
cp ../c-chess-cli.* $1
cp ../games.pgn $1
cat ../c-chess-cli.* > $1log.out
cat $1log.out | grep -E "$2 (-> info|<- ucinewgame|-> depth_debug)" > $1morphe_nn.out
cat $1log.out | grep -E "$3 (-> info|<- ucinewgame|-> depth_debug)" > $1morphe_random.out
