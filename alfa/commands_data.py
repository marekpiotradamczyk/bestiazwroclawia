from dataclasses import dataclass

#########################################################
# Input
@dataclass
class SetOptionCommand(object):
    name : str = ""
    value : str = ""

@dataclass
class PositionCommand(object):
    # The initial value of FEN is set to the standard starting position
    # and later the FEN string is modified to represent desired position
    fen : str =  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    # Sequence of moves played on the chessboard
    moves : list[str] = None

# Class below also represents the default search configuration
@dataclass
class GoCommand(object):
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
# Output
@dataclass
class IdCommand(object):
    name : str = ""
    author : str = ""

@dataclass
class BestMoveCommand(object):
    bestmove : str = ""
    ponder : str = None

@dataclass
class InfoCommand(object):
    depth : int = None
    seldepth : int = None
    time : int = None
    nodes : int = None
    pv : list[str] = None
    multipv : int = 1
    score : tuple[int, int, bool, bool] = None
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