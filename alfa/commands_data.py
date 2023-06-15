from dataclasses import dataclass
import chess
import zorba

#########################################################
# Search
@dataclass
class StartSearchCommand:
    # Modified because the search requires info given with the 'go' command to run the search
    # Which exactly, will be decided later
    searchmoves : list[str] = None
    ponder : bool = False
    wtime : int = None
    btime : int = None
    winc : int = None
    binc : int =  None
    movestogo : int = None
    depth : int = None
    nodes : int = None
    mate : int = None
    movetime : int = None
    infinite : bool = False
    ##
    board : chess.Board = None
    posHash : int = None

#########################################################
# UCI Input
@dataclass
class SetOptionCommand:
    name : str = ""
    value : str = ""

@dataclass
class PositionCommand:
    # The initial value of FEN is set to the standard starting position
    # and later the FEN string is modified to represent desired position
    fen : str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    # Sequence of moves played on the chessboard
    moves : list[str] = None

# Class below also represents the default search configuration
@dataclass
class GoCommand:
    searchmoves : list[str] = None
    ponder : bool = False
    wtime : int = None
    btime : int = None
    winc : int = None
    binc : int =  None
    movestogo : int = None
    depth : int = None
    nodes : int = None
    mate : int = None
    movetime : int = None
    infinite : bool = False

#########################################################
# UCI Output
@dataclass
class IdCommand:
    name : str = ""
    author : str = ""

@dataclass
class BestMoveCommand:
    bestmove : str = ""
    ponder : str = None

@dataclass
class InfoCommand:
    depth : int = None
    seldepth : int = None
    time : int = None
    nodes : int = None
    pv : list[str] = None
    multipv : int = 1
    score : tuple[int, int, bool, bool] = None #  cp, mate, lowerbound, upperbound
    currmove : str = None
    currmovenumber : int = 1 # or 0 
    hashfull : int = None
    nps : int = None
    tbhits : int = None
    sbhits : int = None
    cpuload : int = None
    string : str = ""
    refutation : list[str] = None
    currline : tuple[int, list[str]] = None