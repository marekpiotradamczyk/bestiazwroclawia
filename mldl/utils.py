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


def get_columns_names():
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
