from uci import UCI
from multiprocessing import Process, Pipe
import commands_data as CD
import chess

class Engine(object):
    def __init__(self):
        self.protocol = None
        self.uci = None
        self.uci_process = None
        self.is_searching = False
        self.search_process = None
        self.board = None

    def run(self):
        self._init_protocol()
        self._init_search()
        if self.protocol == 'uci':
            while True:
                # UCI
                # Check if there is anything to receive
                if self.uci_process.poll():
                    # Receive command and args (or more accurately objects)
                    command, args = self.uci_process.recv()
                    match command:
                        case 'isalive':
                            self._isalive_command()
                        case 'position':
                            self._position_command(args)
                        case 'startsearch':
                            self._startsearch_command(args)
                        case 'stopsearch':
                            self._stopsearch_command()
                        case 'quit':
                            break
                # Search
                if self.uci_process.poll():
                    command, args = self.uci_process.recv()
                    if command == 'bestmove':
                        self._bestmove_command(args)
        self._exit()
                

    def _init_protocol(self):
        self.protocol = input.strip()
        if self.protocol == 'uci':
            conn = Pipe()
            self.uci = Process(target=UCI("mario", "test"), args=(conn, ))
            self.uci.start()
    
    def _init_search(self):
        print("Search initialization...")

    def _isalive_command(self):
        self.uci_process.send('isalive', None)

    def _position_command(self, settings : CD.PositionCommand):
        board = chess.Board(settings.fen)
        for move in settings.moves:
            board.push(chess.Move.from_uci(move))
        self.board = board
    
    def _startsearch_command(self, settings : CD.GoCommand):
        self.is_searching = True
        # TODO!
        print("Start searching...")
    
    def _bestmove_command(self, settings : CD.BestMoveCommand):
        new_bestmove = settings.bestmove
        self.uci_process.send('bestmove', CD.BestMoveCommand(bestmove=new_bestmove))

    def _stopsearch_command(self):
        self.is_searching = False
        self.search_process.send()
        print("Stop searching...")

    def _exit(self):
        self.search_process.send('quit', None)
        if self.uci.is_alive():
            self.uci_process.send('quit', None)
        # Wait for all processes to finish