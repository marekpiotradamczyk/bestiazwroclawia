from threading import Thread
from queue import Queue
from uci_input import UCI_Input
from uci_output import UCI_Output
from search import Search
import commands_data as CD
import chess

class Engine:
    def __init__(self):
        self.protocol = None
        self.uci_input = None
        self.uci_output = None
        self.search = None
        self.uci_input_message_queue = Queue()
        self.uci_output_message_queue = Queue()
        self.search_input_message_queue = Queue()
        self.search_output_message_queue = Queue()
        self.is_searching = False
        self.board = None

    def run(self):
        self._init_threads()
        if self.protocol == 'uci':
            while True:
                if not self.uci_input_message_queue.empty():
                    command, args = self.uci_input_message_queue.get()
                    match command:
                        case 'position':
                            self._position_command(args)
                        case 'go':
                            self._startsearch_command(args)
                        case 'stop':
                            self._stopsearch_command()
                        case 'quit':
                            break
                    # self.uci_input_message_queue.task_done()
                if not self.search_output_message_queue.empty():
                    command, args = self.search_output_message_queue.get()
                    if command == 'bestmove':
                        self._bestmove_command(args)
                    # self.search_message_queue.task_done()
            self._exit()

    def _init_threads(self):
        self.protocol = input().strip()
        if self.protocol == 'uci':
            self.uci_input = UCI_Input(author="mario", name="test", input_message_queue=self.uci_input_message_queue, 
                                       output_message_queue=self.uci_output_message_queue)
            self.uci_output = UCI_Output(message_queue=self.uci_output_message_queue)
            self.uci_input.start()
            self.uci_output.start()

            self.search = Search(input_message_queue=self.search_input_message_queue, output_message_queue=self.search_output_message_queue)
            self.search.start()

    def _position_command(self, settings: CD.PositionCommand):
        board = chess.Board(settings.fen)
        for move in settings.moves:
            board.push(chess.Move.from_uci(move))
        self.board = board
        # print(self.board)

    def _startsearch_command(self, settings: CD.GoCommand):
        self.is_searching = True
        startsearch = CD.StartSearchCommand()
        for subcommand in ['searchmoves', 'ponder', 'wtime', 'btime', 'winc', 'binc', 'movestogo',
                       'depth', 'nodes', 'mate', 'movetime', 'infinite']:
            setattr(startsearch, subcommand, getattr(settings, subcommand))
        startsearch.board = self.board
        self.search_input_message_queue.put(['searchposition', startsearch])

    def _bestmove_command(self, settings: CD.BestMoveCommand):
        new_bestmove = settings.bestmove
        self.uci_output_message_queue.put(['bestmove', CD.BestMoveCommand(bestmove=new_bestmove)])

    def _stopsearch_command(self):
        self.is_searching = False
        self.search.stopsearch_command()
        # self.search_input_message_queue.put(['stop', None])

    def _exit(self):
        if self.uci_input.is_alive():
            self.uci_input_message_queue.put(['quit', None])
        if self.uci_output.is_alive():
            self.uci_output_message_queue.put(['quit', None])
        if self.search.is_alive():
            self.search_input_message_queue.put(['quit', None])
        
        self.uci_input.join()
        self.uci_output.join()
        self.search.join()

        # Wait for all processes to finish
        # self.uci_input_message_queue.join()
        # self.uci_output_message_queue.join()
        # # self.search_message_queue.join()
        # self.search_input_message_queue.join()
        # self.search_output_message_queue.join()
        # print("Works just fine!")
