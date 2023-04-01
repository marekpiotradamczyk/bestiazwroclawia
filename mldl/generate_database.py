import argparse
import os
import sys

import chess
import chess.engine
import chess.pgn

import pandas as pd
import numpy as np

from tqdm import trange

from utils import encode_board, get_columns_names


_DATABASE_DIR = "./database"  # Directory for generated files
_PART_SIZE = 1  # Number of games to process in each part

_MATE_SCORE = 1000000  # Score for a mate
_MAX_TIME = 1.0  # Maximum time (in seconds) to use for engine analysis
_MIN_DEPTH = 18  # Minimum depth for engine analysis
_TIME_LIMIT = chess.engine.Limit(time=_MAX_TIME)  # Time limit for engine analysis
_DEPTH_LIMIT = chess.engine.Limit(depth=_MIN_DEPTH)  # Depth limit for engine analysis


def board_score(board, engine):
    """Calculate the score for a given chess board using given engine.

    Args:
        board: A chess.Board object representing the current board position.
        engine: A chess.engine.SimpleEngine object representing the Stockfish engine.
    """
    info = engine.analyse(board, _TIME_LIMIT)
    if info["depth"] < _MIN_DEPTH and not info["score"].is_mate():
        info = engine.analyse(board, _DEPTH_LIMIT)
    return info["score"].relative.score(mate_score=_MATE_SCORE)


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


def _generate_database(pgn, engine, start, size, directory):
    """Generate a part of the database by processing a set of games from given PGN file.

    Args:
        pgn: An open file object representing the PGN file to process.
        engine: A chess.engine.SimpleEngine object representing the Stockfish engine.
        start: The number of the first game to process.
        size: The number of games to process.
        directory: The directory path to save generated CSV files.
    """
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
            score = board_score(board, engine)
            row = encode_board(board)
            row.append(score)
            if board.turn == chess.WHITE:
                white.append(row)
            else:
                black.append(row)

    columns = get_columns_names()
    columns.append("score")

    if len(white) > 0:
        white_filename = f"{directory}/white_{start}-{start+i}.csv"
        pd.DataFrame(white, columns=columns, dtype=int).to_csv(
            white_filename, header=True, index=False
        )

    if len(black) > 0:
        black_filename = f"{directory}/black_{start}-{start+i}.csv"
        pd.DataFrame(black, columns=columns, dtype=int).to_csv(
            black_filename, header=True, index=False
        )


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
        default="stockfish_15.1_linux_x64_avx2/stockfish-ubuntu-20.04-x86-64-avx2",
        help=f"Path to stockfish binary. Default is 'stockfish_15.1_linux_x64_avx2/stockfish-ubuntu-20.04-x86-64-avx2'",
    )
    parser.add_argument(
        "-d",
        "--database",
        type=str,
        default=f"{_DATABASE_DIR}/lichess_db.pgn",
        help=f"Path to lichess database in pgn format. Default is '{_DATABASE_DIR}/lichess_db.pgn'",
    )
    parser.add_argument(
        "-o",
        "--output",
        type=str,
        default=_DATABASE_DIR,
        help=f"Directory to save generated CSV files. Default is '{_DATABASE_DIR}'",
    )
    args = parser.parse_args()

    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    # check if required files and directories exist
    _check_if_file_exists(args.stockfish)
    _check_if_file_exists(args.database)
    if not os.path.exists(args.output):
        os.mkdir(args.output)

    # run Stockfish and open file with games
    engine = chess.engine.SimpleEngine.popen_uci(args.stockfish)
    pgn = open(args.database)

    games_to_drop = _largest_number_of_available_game(args.output)
    for _ in range(games_to_drop):
        chess.pgn.read_game(pgn)

    start = games_to_drop + 1
    for part in range(args.parts):
        print(f"Generating part {part + 1}/{args.parts}")
        _generate_database(pgn, engine, start, _PART_SIZE, args.output)
        start += _PART_SIZE

    pgn.close()
    engine.close()
    print("DONE!")
