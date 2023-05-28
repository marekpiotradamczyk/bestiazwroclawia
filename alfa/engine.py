from threading import Thread
from queue import Queue
from queue import Queue
from uci import UCI
import commands_data as CD
import chess

class Engine:
    def __init__(self):
        self.protocol = None
        self.uci = None  
        self.uci_message_queue = Queue()
        self.search = None
        self.search_message_queue = Queue()
        self.is_searching = False
        self.board = None

    def run(self):
        self._init_threads()
        if self.protocol == 'uci':
            while True:
                # UCI
                # Check if there is anything to receive
                if not self.uci_message_queue.empty():
                    # Receive command and args (or more accurately objects)
                    command, args = self.uci_message_queue.get()
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
                    self.uci_message_queue.task_done()
                # Search
                if not self.search_message_queue.empty():
                    command, args = self.uci_message_queue.get()
                    if command == 'bestmove':
                        self._bestmove_command(args)
                    self.search_message_queue.task_done()
        self._exit()
                

    def _init_threads(self):
        self.protocol = input.strip()
        if self.protocol == 'uci':
            self.uci = UCI(author="mario", name="test", message_queue=self.uci_message_queue)
            self.uci.start()
        # self.search = Search(message_queue=self.search_message_queue)
        # self.search.start()

    def _isalive_command(self):
        self.uci_message_queue.put(['isalive', None])

    def _position_command(self, settings : CD.PositionCommand):
        board = chess.Board(settings.fen)
        for move in settings.moves:
            board.push(chess.Move.from_uci(move))
        self.board = board
    
    def _startsearch_command(self, settings : CD.GoCommand):
        # SEARCH!
        self.is_searching = True
        print("Start searching...")
    
    def _bestmove_command(self, settings : CD.BestMoveCommand):
        new_bestmove = settings.bestmove
        self.uci_message_queue.put(['bestmove', CD.BestMoveCommand(bestmove=new_bestmove)])

    def _stopsearch_command(self):
        # SEARCH!
        self.is_searching = False
        self.search_message_queue.put(['stop', None])
        print("Stop searching...")

    def _exit(self):
        self.search_message_queue.put(['quit', None])
        if self.uci.is_alive():
            self.uci_message_queue.put(['quit', None])
        # Wait for all processes to finish
        self.uci_message_queue.join()
        self.search_message_queue.join()