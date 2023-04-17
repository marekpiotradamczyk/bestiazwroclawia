import argparse
import os
import sys
import time

import chess
import chess.engine
import chess.pgn

import pandas as pd
import numpy as np

from tqdm import trange

from utils import encode_board, get_columns_names


_DATABASE_DIR = "./database"  # Directory for generated files
_PART_SIZE = 100  # Number of games to process in each part

_MATE_SCORE = 1000000  # Score for a mate
_MAX_TIME = 1.0  # Maximum time (in seconds) to use for engine analysis
_MIN_DEPTH = 18  # Minimum depth for engine analysis
_TIME_LIMIT = chess.engine.Limit(time=_MAX_TIME)  # Time limit for engine analysis
_DEPTH_LIMIT = chess.engine.Limit(depth=_MIN_DEPTH)  # Depth limit for engine analysis

_SCRIPT_PATH = os.path.dirname(os.path.abspath(__file__))  # Path to current script
_DEFAULT_DATABASE_PATH = f"{_SCRIPT_PATH}/database"  # Default path to database
_DEFAULT_STOCKFISH_PATH = f"{_SCRIPT_PATH}/stockfish_15.1_linux_x64_avx2/stockfish-ubuntu-20.04-x86-64-avx2"  # Default path to stockfish

_DEFAULT_DATABASE_REL_PATH = os.path.relpath(
    _DEFAULT_DATABASE_PATH, os.getcwd()
)  # Default relative path to database
_DEFAULT_STOCKFISH_REL_PATH = os.path.relpath(
    _DEFAULT_STOCKFISH_PATH, os.getcwd()
)  # Default relative path to stockfish


games_count = 0
positions_count = 0


def _print_statistics(seconds):
    global games_count
    global positions_count
    print()
    print("===STATS===")
    print(f"total time:      {seconds:.2f}s")
    print(f"total positions: {positions_count}")
    print(f"total games:     {games_count}")
    print(f"positions/s:     {positions_count / seconds:.2f}")
    print(f"games/s:         {games_count / seconds:.2f}")
    print()


def board_score_and_best_move(board, engine):
    """Calculate the score and best move for a given chess board using given engine.

    Args:
        board: A chess.Board object representing the current board position.
        engine: A chess.engine.SimpleEngine object representing the Stockfish engine.
    """
    info = engine.analyse(board, _TIME_LIMIT)
    if info["depth"] < _MIN_DEPTH and not info["score"].is_mate():
        info = engine.analyse(board, _DEPTH_LIMIT)
    pv = info.get("pv")
    best_move = pv[0].uci() if pv is not None and len(pv) > 0 else ""
    score = info["score"].relative.score(mate_score=_MATE_SCORE)
    return score, best_move


def _largest_number_of_available_game(directory):
    """Finds the largest game number in the existing database files."""
    largest_number = 0
    for filename in os.listdir(directory):
        if filename.endswith(".csv"):
            try:
                number = int(filename[:-4].split("-")[-1])
                largest_number = max(largest_number, number)
            except ValueError:
                continue
    return largest_number


def _generate_database(pgn, engine, start_pos, size, directory):
    """Generate a part of the database by processing a set of games from given PGN file.

    Args:
        pgn: An open file object representing the PGN file to process.
        engine: A chess.engine.SimpleEngine object representing the Stockfish engine.
        start_pos: The number of the first game to process.
        size: The number of games to process.
        directory: The directory path to save generated CSV files.
    """
    global games_count
    global positions_count

    white = []
    black = []
    for i in trange(size):
        game = chess.pgn.read_game(pgn)
        if game is None:
            print("EOF reached", file=sys.stderr)
            break
        board = game.board()
        for move in game.mainline_moves():
            board.push(move)
            score, best_move = board_score_and_best_move(board, engine)
            row = encode_board(board)
            row.append(score)
            row.append(best_move)
            row.append(start_pos + i)
            if board.turn == chess.WHITE:
                white.append(row)
            else:
                black.append(row)
            positions_count += 1
        games_count += 1

    board_columns = get_columns_names()
    all_columns = board_columns + ["score", "best_move", "game_id"]

    if len(white) > 0:
        white_filename = f"{directory}/white_{start_pos}-{start_pos+i}.csv"
        df = pd.DataFrame(white, columns=all_columns)
        df[board_columns] = df[board_columns].astype(int)
        df.to_csv(white_filename, header=True, index=False)

    if len(black) > 0:
        black_filename = f"{directory}/black_{start_pos}-{start_pos+i}.csv"
        df = pd.DataFrame(black, columns=all_columns)
        df[board_columns] = df[board_columns].astype(int)
        df.to_csv(black_filename, header=True, index=False)


def _check_if_file_exists(filename):
    """Checks if the given file exists and abort if not."""
    if not os.path.isfile(filename):
        print(f"File {filename} does not exist!", file=sys.stderr)
        exit(2)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-p",
        "--parts",
        type=int,
        default=1,
        help=f"Number of parts to generate. Deafult is 1. Each part consists of {_PART_SIZE} games.",
    )
    parser.add_argument(
        "-s",
        "--stockfish",
        type=str,
        default=_DEFAULT_STOCKFISH_REL_PATH,
        help=f"Path to stockfish binary. Default is '{_DEFAULT_STOCKFISH_REL_PATH}'",
    )
    parser.add_argument(
        "-d",
        "--database",
        type=str,
        default=f"{_DEFAULT_DATABASE_REL_PATH}/lichess_db.pgn",
        help=f"Path to lichess database in pgn format. Default is '{_DEFAULT_DATABASE_REL_PATH}/lichess_db.pgn'",
    )
    parser.add_argument(
        "-o",
        "--output",
        type=str,
        default=_DEFAULT_DATABASE_REL_PATH,
        help=f"Directory to save generated CSV files. Default is '{_DEFAULT_DATABASE_REL_PATH}'",
    )
    parser.add_argument(
        "--stdin",
        default=False,
        action="store_true",
        help="Read database from standard input. This option overrides -d flag.",
    )
    args = parser.parse_args()

    # check if required files and directories exist
    _check_if_file_exists(args.stockfish)
    if not args.stdin:
        _check_if_file_exists(args.database)
    if not os.path.exists(args.output):
        os.mkdir(args.output)

    # run Stockfish and open file with games
    engine = chess.engine.SimpleEngine.popen_uci(args.stockfish)
    if args.stdin:
        pgn = sys.stdin
    else:
        pgn = open(args.database)

    # ignore games which already exist in database
    games_to_drop = _largest_number_of_available_game(args.output)
    for _ in range(games_to_drop):
        chess.pgn.skip_game(pgn)

    start_time = time.time()

    start_pos = games_to_drop + 1
    for part in range(args.parts):
        print(f"Generating part {part + 1}/{args.parts}")
        _generate_database(pgn, engine, start_pos, _PART_SIZE, args.output)
        start_pos += _PART_SIZE

    pgn.close()
    engine.close()

    total_time = time.time() - start_time
    _print_statistics(total_time)
    print("DONE!")
