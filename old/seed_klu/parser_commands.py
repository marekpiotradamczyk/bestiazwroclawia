from dataclasses import dataclass, field

@dataclass
class QuitCommand(object):
    pass

# *************************************
# Commands used in communication
# UCIInputParser process -> UCI process
# *************************************
@dataclass
class UCICommand(object):
    pass

@dataclass
class IsReadyCommand(object):
    pass

@dataclass
class StopCommand(object):
    pass

@dataclass
class PositionCommand(object):
    fen : str = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
    moves : list = field(default_factory=list)

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

# **************************************
# Commands used in communication
# UCI process -> UCIOutputParser process
# **************************************
@dataclass
class IDCommand(object):
    name : str = ""
    author : str = ""

@dataclass
class UCIOkCommand(object):
    pass

@dataclass
class ReadyOkCommand(object):
    pass

@dataclass
class BestMoveCommand(object):
    bestmove : str
    ponder : str = None