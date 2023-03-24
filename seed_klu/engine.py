from multiprocessing import Process, Pipe
from parser_commands import QuitCommand
from uci import UCI
import parser_commands as PC
import uci_commands as UC
import engine_commands as EC
import search_commands as SC
import logging
from search import SearchProcess
from time_management import TimeManager
import chess

class Engine(object):
    def __init__(self):
        self.uci = None
        #two way
        self.uci_pipe = None
        self.protocol = None
        #two way
        self.search_pipe = None
        self.search_process = None
        self.searching = False
        self.position = None
        self.time_manager = TimeManager()
        logging.info("Engine ready!")

    def engine_loop(self):
        self._set_protocol()
        self._set_search()
        if self.protocol == "uci":
            while True:
                if self.uci_pipe.poll():
                    msg = self.uci_pipe.recv()
                    match msg:
                        case QuitCommand():
                            break
                        case UC.IsAliveCommand():
                            self._handle_isalive_command()
                        case UC.SetPositionCommand():
                            self._handle_setposition_command(msg)
                        case UC.StartSearchCommand():
                            self._handle_startsearch_command(msg)
                        case UC.StopSearchCommand():
                            self._stop_search()

                if self.search_pipe.poll():
                    msg = self.search_pipe.recv()
                    match msg:
                        case SC.BestMoveCommand():
                            self._handle_bestmove_command(msg)
                
                if self.searching and self.time_manager.out_of_time():
                    self._stop_search()
        self._handle_exit()

    def _handle_exit(self):
        self.search_pipe.send(QuitCommand())
        if self.uci.is_alive():
            self.uci_pipe.send(QuitCommand())
        self.search_process.join()
        self.uci.join()

    def _handle_isalive_command(self):
        self.uci_pipe.send(EC.AliveCommand())

    def _handle_setposition_command(self, command : UC.SetPositionCommand):
        board = chess.Board(command.fen)
        for mv in command.moves:
            board.push(chess.Move.from_uci(mv))
        self.position = board
    
    def _handle_startsearch_command(self, command : UC.StartSearchCommand):
        logging.critical(command)
        self.searching = True
        if command.infinite:
            self.time_manager.set_timer(0, True)
        elif command.movetime != None:
            self.time_manager.set_timer(command.movetime, False)
        else:
            if self.position.turn == chess.WHITE:
                if command.wtime != None:
                    self.time_manager.set_timer(command.wtime//60, False)
                else:
                    self.time_manager.set_timer(0, True)
            else:
                if command.btime != None:
                    self.time_manager.set_timer(command.btime//60, False)
                else:
                    self.time_manager.set_timer(0, True)
        self.search_pipe.send(SC.SearchPositionCommand(self.position))

    def _stop_search(self):
        self.searching = False
        self.search_pipe.send(SC.StopSearchCommand())

    def _handle_bestmove_command(self, command : SC.BestMoveCommand):
        self.uci_pipe.send(EC.BestMoveCommand(bestmove=chess.Move.uci(command.move)))

    def _set_protocol(self):
        self.protocol = input().strip()
        if self.protocol == "uci":
            conn1, conn2 = Pipe(duplex=True)
            self.uci_pipe = conn1
            self.uci = UCI("Krzysztof Lukasiewicz", "Bad Mother Fucker", conn2)
            self.uci.start()

    def _set_search(self):
        conn1, conn2 = Pipe(duplex=True)
        self.search_pipe = conn1
        self.search_process = SearchProcess(conn2)
        self.search_process.start()
            
    