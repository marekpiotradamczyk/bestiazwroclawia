#include<iostream>

#include "parser.h"
#include "position.h"
#include "search.h"
#include "types.h"

int main(){
  srand(1232111);
  std::ios::sync_with_stdio(false);
  std::cout.setf(std::ios::unitbuf);
  int time_left, opponent_time_left;
  Position position;
  position.Init();
  Search search;
  std::string comm, out;

  bool move_parsed = false;
  while(true){ // game loop
    std::cout << "# inside game loop\n";
    getline(std::cin, comm);
    std::cout << "# " << comm << "\n";
    // std::cout << "# " << comm << "\n";
    if(comm == "xboard"){
      std::cout << "feature reuse=0\n";
      std::cout << "feature sigint=0\n";
      std::cout << "feature name=Sweeper\n";
      std::cout << "feature variants=\"crazyhouse\"\n";
      std::cout << "feature debug=1\n";
      std::cout << "feature done=1\n";
    }
    else if(comm == "quit")
        return 0;
    else if(comm == "new"){
      position.NewGame();
    }
    else if(comm == "draw"){
        std::cout << "offer draw";
    }
    else if(comm.substr(0,5) == "level"){
        // TODO: parse level command and time
        continue;
    }
    else if(comm == "post"){
        // TODO: parse level command and time
        continue;
    }
    else if(comm == "hard"){
      // TODO: implement pondering
      continue;
    }
    else if(comm == "white"){
      continue;
    }
    else if(comm == "black"){
      continue;
    }
    else if(comm == "force"){
      // TODO: implement force mode
      continue;
    }
    else if(comm == "go"){
      move_parsed = true;
    }
    else if(comm.substr(0,7) == "variant" && comm.substr(8, 10) == "crazyhouse"){
      position.crazyhouse = true;
    }
    else if(comm.substr(0,4) == "time"){
        time_left = stoi(comm.substr(5));
        std::cout << "#       my time remaining: " << time_left/100 << "seconds\n";
        search.time_left = time_left;
    }
    else if(comm.substr(0,4) == "otim"){
        opponent_time_left = stoi(comm.substr(5));
        std::cout << "# opponent time remaining: " << opponent_time_left/100 << "seconds\n";
    }
    else if(comm.length() == 4 || comm.length() == 5){
      std::cout << "# move received\n";
      std::cout << "# " << comm << "\n";
    	MoveType move = StringToMove(comm);
    	position.MakeMove(move);
      move_parsed = true;
    }
    else{
      continue;
    }
    if(move_parsed){
      MoveType move = search.BestMove(position);
      position.MakeMove(move);
      out = MoveToString(move);
      std::cout << "move " << out << "\n";
      move_parsed = false;
    }
  }

  return 0;
}
