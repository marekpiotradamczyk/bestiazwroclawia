#!/bin/bash
cat ../c-chess-cli.* > log.out
cat log.out | grep -E "morphebot_nn (-> info|<- ucinewgame|-> depth_debug)" > morphe_nn.out
cat log.out | grep -E "morphebot_random (-> info|<- ucinewgame|-> depth_debug)" > morphe_random.out
