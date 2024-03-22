from dataclasses import dataclass
# *************************************
# Commands used in communication
# UCI process -> Main process
# *************************************
@dataclass
class IsAliveCommand(object):
    pass

@dataclass
class StopSearchCommand(object):
    pass

@dataclass
class SetPositionCommand(object):
    fen : str
    moves : list

@dataclass
class StartSearchCommand(object):
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
