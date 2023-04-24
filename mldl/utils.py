import os
import pandas as pd
import numpy as np
import chess


PIECES = [chess.PAWN, chess.KNIGHT, chess.BISHOP, chess.ROOK, chess.QUEEN, chess.KING]
PLAYERS = [chess.WHITE, chess.BLACK]


def encode_board(board):
    """
    Encodes a chess board as a list of binary values representing the presence of pieces and players' castling rights.

    Args:
        board: A chess.Board object representing the current state of the game.

    Returns:
        A list of binary values representing the presence of pieces and players' castling rights on the board. The
        encoding includes 64 values for each piece type and player (1 if the piece is present, 0 if it's not), as well
        as a boolean value indicating whether each player has kingside and queenside castling rights.
    """
    encoding = []
    for player in PLAYERS:
        for piece in PIECES:
            encoding.extend(board.pieces(piece, player).tolist())
        encoding.append(board.has_kingside_castling_rights(player))
        encoding.append(board.has_queenside_castling_rights(player))
    return encoding


def get_board_columns_names():
    """
    Returns a list of column names that can be used to label the encoded data from the `encode_board` function.

    Returns:
        A list of column names that includes the symbol and position of each piece on the board, as well as the kingside
        and queenside castling rights for each player. The labels for the castling rights are simply "K" and
        "Q", which represent the kingside and queenside castling rights, respectively. The column names for each player
        are capitalized for white and lowercase for black.
    """
    fix_names_func = {
        chess.WHITE: lambda x: x.upper(),
        chess.BLACK: lambda x: x.lower(),
    }
    columns = []
    for player in PLAYERS:
        for piece in PIECES:
            symbol = fix_names_func[player](chess.piece_symbol(piece))
            columns.extend(map(lambda square: f"{square}{symbol}", chess.SQUARE_NAMES))
        columns.append(fix_names_func[player]("K"))
        columns.append(fix_names_func[player]("Q"))
    return columns


def _read_and_merge_files(files):
    """Read and merge multiple CSV files containing chess board positions and scores and remove duplicates."""
    board_columns = get_board_columns_names()
    columns_without_move = board_columns + ["score", "game_id"]
    types_dict = {col: bool for col in board_columns}
    frames = []
    for filename in files:
        frames.append(pd.read_csv(filename, dtype=types_dict))

    return (
        pd.concat(frames)
        .dropna(subset=columns_without_move)    # NaNs are possible in 'best_move' column (in checkmate positions)
        .drop_duplicates(subset=board_columns)
        .fillna("")     # fill NaN from 'best_move' with empty string
        .reset_index()
    )


def read_database(directory="./database"):
    """Read from given directory all CSV files containing chess board positions and scores and remove duplicates."""

    def file_matcher(prefix):
        return lambda file: file.startswith(prefix) and file.endswith(".csv")

    def full_name(file):
        return f"{directory}/{file}"

    white_files = map(full_name, filter(file_matcher("white_"), os.listdir(directory)))
    black_files = map(full_name, filter(file_matcher("black_"), os.listdir(directory)))

    return _read_and_merge_files(white_files), _read_and_merge_files(black_files)


def train_test_split_by_game_id(white, black, test_size=0.3):
    """Split dataframes into traininig and testing sets using the 'game_id' column as the grouping variable."""
    unique_game_ids = white["game_id"].unique()
    num_test_games = int(test_size * len(unique_game_ids))
    selected_test_game_ids = np.random.choice(unique_game_ids, size=num_test_games, replace=False)

    white_test_indices = white["game_id"].isin(selected_test_game_ids)
    black_test_indices = black["game_id"].isin(selected_test_game_ids)

    white_test = white[white_test_indices]
    white_train = white[~white_test_indices]
    black_test = black[black_test_indices]
    black_train = black[~black_test_indices]
    return white_train, white_test, black_train, black_test


def filter_dataset(df, score_threshold=50):
    """Filter the input dataframe based on the score threshold."""
    return df[(df["score"] >= score_threshold) | (df["score"] <= -score_threshold)]


def get_features_and_labels(df):
    """Get the features and labels from the input dataframe."""
    return df[get_board_columns_names()], df["score"] > 0
