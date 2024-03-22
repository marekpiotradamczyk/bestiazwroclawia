from multiprocessing import Process
from multiprocessing.connection import Connection
import sys
from parser_commands import *
import logging


def _is_command(s : str) -> bool:
    return s in ['quit', 'uci', 'isready', 'position', 'go', 'stop']
#['quit', 'uci', 'debug', 'isready', 'setoption', 'ucinewgame', 'position', 'go', 'stop', 'ponderhit']

def _separate_command(line : str) -> tuple[str, str]:
    f = line.find(' ')
    return (line, "") if f == -1 else (line[:f], line[f+1:].strip())

def _get_command_and_argument(line : str) -> tuple[str, str]:
    command = line
    while command != "" and not _is_command(command):
        command, line = _separate_command(line)
    return command, line

class UCIInputParser(Process):
    def __init__(self, connection : Connection):
        Process.__init__(self)
        # One way connection from UCIInputParser process to UCI process
        self.connection = connection
        logging.info("UCIInputParser ready!")
    
    def run(self):
        sys.stdin = open(0)
        while True:
            try:
                line = input().strip()
            except EOFError:
                break
            command, argument = _get_command_and_argument(line)
            if command == "":
                continue
            elif command == "quit":
                break
            else:
                getattr(self, "_handle_"+command+"_command")(argument)
        self._handle_quit_command()
        

    def _handle_quit_command(self, argument : str = str()):
        logging.info("Quitting UCI input parser")
        self.connection.send(QuitCommand())
            
    def _handle_uci_command(self, argument : str):
        self.connection.send(UCICommand())

    def _handle_isready_command(self, argument : str):
        self.connection.send(IsReadyCommand())

    def _handle_position_command(self, argument : str):
        mvid = argument.find('moves')
        if mvid != -1:
            fen = argument[:mvid].strip()
            moves = argument[mvid+len('moves'):].split()
        else:
            fen = argument
            moves = list()
        if fen != 'startpos':
            self.connection.send(PositionCommand(fen=fen, moves=moves))
        else:
            self.connection.send(PositionCommand(moves=moves))
    
    def _handle_go_command(self, argument : str):
        is_subcommand = lambda s: s in ['searchmoves', 'ponder', 'wtime', 'btime', \
                                        'winc', 'binc', 'movestogo', 'depth', 'nodes', \
                                        'mate', 'movetime', 'infinite']
        is_integer_command = lambda s : s in ['wtime', 'btime', 'winc', 'binc', 'movestogo', \
                                              'depth', 'nodes', 'mate', 'movetime']
        def is_chess_move(s : str) -> bool:
            is_row = lambda x: ord(x) in range(ord('1'), ord('9'))
            is_col = lambda x: ord(x) in range(ord('a'), ord('i'))
            s = s.lower()
            #nullmove
            if s == "0000":
                return True
            if len(s) == 4:
                return is_col(s[0]) and is_row(s[1]) and is_col(s[2]) and is_row(s[3])
            if len(s) == 5:
                return is_col(s[0]) and is_row(s[1]) and is_col(s[2]) and is_row(s[3]) \
                       and s[4] in ['q', 'b', 'n', 'r'] #promotion of a pawn
            return False

        #simple but verbose way to implement it :/
        tokens = argument.split()
        c = GoCommand()
        for i, token in enumerate(tokens):
            if not is_subcommand(token):
                continue
            elif is_integer_command(token) and i+1 < len(tokens) and tokens[i+1].isnumeric():
                setattr(c, token, int(tokens[i+1]))
            elif token == "searchmoves":
                c.searchmoves = []
                for t in tokens[i+1:]:
                    if is_subcommand(t):
                        break
                    if is_chess_move(t):
                        c.searchmoves.append(t)
            elif token == "ponder":
                c.ponder = True
            elif token == "infinite":
                c.infinite = True
        self.connection.send(c)

    def _handle_stop_command(self, argument : str):
        self.connection.send(StopCommand())