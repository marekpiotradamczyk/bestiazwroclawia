from multiprocessing import Process, Pipe
from multiprocessing.connection import Connection
from uci_input_parser import UCIInputParser
from uci_output_parser import UCIOutputParser
from parser_commands import QuitCommand
import parser_commands as PC
import uci_commands as UC
import engine_commands as EC
import logging

class UCI(Process):
    def __init__(self, author : str, engine_name : str, connection : Connection):
        Process.__init__(self)
        self.author = author
        self.engine_name = engine_name
        #two way
        self.connection = connection
        self.input_parser = None
        #one way
        self.input_pipe = None
        self.output_parser = None
        #one way
        self.output_pipe = None
        logging.info("UCI process ready!")

    def run(self):
        self._setup_pipe_connection()
        self._send_initial_info()
        
        while True:
            if self.input_pipe.poll():
                msg = self.input_pipe.recv()
                logging.debug(msg)
                match msg:
                    case QuitCommand():
                        break
                    case PC.IsReadyCommand():
                        self.connection.send(UC.IsAliveCommand())
                    case PC.PositionCommand():
                        self.connection.send(UC.SetPositionCommand(fen=msg.fen, moves=msg.moves))
                    case PC.GoCommand():
                        self._handle_go_command(msg)
                    case PC.StopCommand():
                        self.connection.send(UC.StopSearchCommand())
            if self.connection.poll():
                msg = self.connection.recv()
                logging.debug(msg)
                match msg:
                    case QuitCommand():
                        break
                    case EC.AliveCommand():
                        self.output_pipe.send(PC.ReadyOkCommand())
                    case EC.BestMoveCommand():
                        self.output_pipe.send(PC.BestMoveCommand(bestmove=msg.bestmove, ponder=msg.ponder))

        self._handle_exit()
                        
    def _handle_exit(self):
        logging.info("Quitting UCI")
        self.output_pipe.send(QuitCommand())
        self.connection.send(QuitCommand())
        self.input_parser.join()
        self.output_parser.join()

    def _handle_go_command(self, command : PC.GoCommand):
        c = UC.StartSearchCommand()
        for name in ['searchmoves', 'ponder', 'wtime', 'btime', 'winc', \
                     'binc', 'movestogo', 'depth', 'nodes', 'mate', 'movetime', 'infinite']:
            setattr(c, name, getattr(command, name))
        self.connection.send(c)
        

    def _setup_pipe_connection(self):
        conn1, conn2 = Pipe()
        self.input_pipe = conn2
        self.input_parser = UCIInputParser(conn1)
        self.input_parser.start()
        conn1, conn2 = Pipe()
        self.output_pipe = conn1
        self.output_parser = UCIOutputParser(conn2)
        self.output_parser.start()

    def _send_initial_info(self):
        self.output_pipe.send(PC.IDCommand(name=self.engine_name, author=self.author))
        self.output_pipe.send(PC.UCIOkCommand())
