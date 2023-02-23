from dataclasses import dataclass
# *************************************
# Commands used in communication
# Main process -> UCI Process
# *************************************

@dataclass
class AliveCommand(object):
    pass

@dataclass
class BestMoveCommand(object):
    bestmove : str
    ponder : str = None

#empty for now
@dataclass
class InfoCommand(object):
    pass
