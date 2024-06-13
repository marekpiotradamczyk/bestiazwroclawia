#include <iostream>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

#include "chess.hpp"
#include "engine.hpp"
#include "search.hpp"
#include "uci.hpp"

namespace uci {

Uci::Uci(chess::Engine &engine) { engine = engine; }

void Uci::loop() {
  std::string input;

  while (std::getline(std::cin, input)) {
    std::stringstream iss(input);
    std::vector<std::string> tokens;
    std::string token;
    while (iss >> token) {
      tokens.push_back(token);
    }

    if (tokens.empty()) {
      continue;
    }
    if (tokens[0] == "isready") {
      std::cout << "readyok\n";
    } else if (tokens[0] == "uci") {
      std::cout << "BestiaZWroclawia v0.1\n";
    } else if (tokens[0] == "quit") {
      exit(0);
    } else if (tokens[0] == "stop") {
      std::cout << "stop\n";
    } else if (tokens[0] == "position") {
      parsePosition(tokens);
    } else if (tokens[0] == "go") {
      parseGo(tokens);
    } else if (tokens[0] == "setoption") {
      parseEngineOption(tokens);
    } else {
      std::cout << "Unknown command\n";
    }
  }
}

void Uci::parseGo(std::vector<std::string> tokens) {
  int depth = 1;
  if (tokens.size() > 3 && tokens[1] == "depth") {
    depth = stoi(tokens[2]);
  }
  chess::Move move;
  chess::I32 x =
      chess::search(engine.getBoard(), depth, move, INT32_MIN, INT32_MAX);

  std::cout << "bestmove " << chess::uci::moveToUci(move) << "\n";
}

void Uci::parsePosition(std::vector<std::string> tokens) {
  // TODO: if someone's ambitious, make all this format check with one big
  // regex,
  // instead of regex only for FEN

  // Check if tokens are valid
  if (tokens.size() < 2 || (tokens[1] != "startpos" && tokens[1] != "fen")) {
    std::cerr << "Invalid input format!" << std::endl;
    return;
  }

  int parsing_index = 2; // last token position that was parsed + 1
  std::string fen = chess::constants::STARTPOS;

  if (tokens[1] == "fen") {
    if (tokens.size() < 8) {
      std::cerr << "Missing FEN string!" << std::endl;
      return;
    }
    std::string given_fen = tokens[2] + " " + tokens[3] + " " + tokens[4] +
                            " " + tokens[5] + " " + tokens[6] + " " + tokens[7];
    if (!isFENValid(given_fen)) {
      std::cerr << "Invalid FEN string!" << std::endl;
      return;
    }
    fen = given_fen;
    parsing_index = 8;
  }

  chess::Board board = chess::Board(fen);

  if (tokens.size() <= parsing_index) {
    engine.setBoard(board);
    return; // we're done
  }
  // now if there's more to parse then its moves
  if (tokens[parsing_index] != "moves") {
    std::cerr << "Missing moves keyword" << std::endl;
    return;
  }

  chess::Move move;
  for (int i = parsing_index + 1; i < tokens.size(); i++) {
    move = chess::uci::uciToMove(board, tokens[i]);
    board.makeMove(move);
  }

  // finally we're done...
  engine.setBoard(board);
}

void Uci::parseEngineOption(std::vector<std::string> tokens) {
  // TODO: parse options and set them for engine
}

bool Uci::isFENValid(const std::string &fen) {
  std::regex fen_regex(
      R"(^(((?:[rnbqkpRNBQKP1-8]+\/){7})[rnbqkpRNBQKP1-8]+)\s([b|w])\s(-|K?Q?k?q)\s(-|[a-h][1-8])\s(\d+\s\d+)$)");
  return std::regex_match(fen, fen_regex);
}

} // namespace uci