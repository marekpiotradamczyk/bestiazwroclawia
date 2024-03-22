from multiprocessing import Process
from multiprocessing.connection import Connection
import sys
from parser_commands import *
import logging



class UCIOutputParser(Process):
    def __init__(self, connection : Connection):
        Process.__init__(self)
        #one way connection from UCI process to UCIOutputParser process
        self.connection = connection
        logging.info("UCIOutputParser ready!")

    def run(self):
        
        while True:
            if self.connection.poll():
                msg = self.connection.recv()
                logging.debug(msg)
                match msg:
                    case QuitCommand():
                        break
                    case UCIOkCommand():
                        self._handle_uciok_command(msg)
                    case IDCommand():
                        self._handle_id_command(msg)
                    case ReadyOkCommand():
                        self._handle_readyok_command(msg)
                    case BestMoveCommand():
                        self._handle_bestmove_command(msg)
        logging.info("Quitting UCI output parser")
                    
    def _handle_uciok_command(self, msg : UCIOkCommand):
        print("uciok", flush=True)
        
    def _handle_id_command(self, msg : IDCommand):
        print(f"name {msg.name}", flush=True)
        print(f"author {msg.author}", flush=True)

    def _handle_readyok_command(self, msg : ReadyOkCommand):
        print("readyok", flush=True)
    
    def _handle_bestmove_command(self, msg : BestMoveCommand):
        s = f"bestmove {msg.bestmove}"
        if msg.ponder != None:
            s += f" ponder {msg.ponder}"
        print(s, flush=True)