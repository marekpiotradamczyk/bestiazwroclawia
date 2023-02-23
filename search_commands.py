from dataclasses import dataclass
import chess

# *************************************
# Commands used in communication
# Engine process -> Search process
# *************************************
@dataclass
class SearchPositionCommand(object):
    position : chess.Board

@dataclass
class StopSearchCommand(object):
    pass

# *************************************
# Commands used in communication
# Search process -> Engine process
# *************************************
@dataclass
class BestMoveCommand(object):
    move : chess.Move
