from threading import Thread
from queue import Queue
# from uci import UCI
import commands_data as CD
import chess
import search


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
        # print(self.protocol)
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
                if not self.search_message_queue.empty():  # ['bestmove', obj : BestMoveCommand]
                    command, args = self.search_message_queue.get()
                    if command == 'bestmove':
                        self._bestmove_command(args)
                    self.search_message_queue.task_done()
        self._exit()

    def _init_threads(self):
        self.protocol = input().strip()
        # if self.protocol == 'uci':
        # self.uci = UCI(author="mario", name="test", message_queue = self.uci_message_queue)
        # self.uci.start()
        GO = CD.GoCommand()
        GO.depth = 7
        GO.movetime = 20

        self._startsearch_command(GO)

    def _isalive_command(self):
        self.uci_message_queue.put(['isalive', None])

    def _position_command(self, settings: CD.PositionCommand):
        board = chess.Board(settings.fen)
        if settings.moves is not None:
            for move in settings.moves:
                board.push(chess.Move.from_uci(move))
        self.board = board

    def _startsearch_command(self, settings: CD.GoCommand):
        # SEARCH!
        self.is_searching = True
        print("Start searching...")
        self._position_command(CD.PositionCommand())
        SSC = CD.StartSearchCommand(self.board, settings.depth, settings.movetime)
        self.search = search.Search(message_queue=self.search_message_queue)
        self.search_message_queue.put(["SearchPosition", SSC])
        bestmove = self.search.run()
        self.search_message_queue.put(bestmove)

    def _bestmove_command(self, settings: CD.BestMoveCommand):

        new_bestmove = settings.bestmove
        # self.uci_message_queue.put(['bestmove', CD.BestMoveCommand(bestmove=new_bestmove)])

    def _stopsearch_command(self):
        # SEARCH! KOMUNIKACJA MIĘDZY THREDAMI HELP!! - W PONIEDZIAŁEK "może działać ale ciężko sprawdzić"
        self.is_searching = False
        self.search_message_queue.put(['stop', None])
        print("Stop searching...")

    def _exit(self):
        self.search_message_queue.put(['quit', None])
        '''
        if self.uci.is_alive():
            self.uci_message_queue.put(['quit', None])
        # Wait for all processes to finish
        self.uci_message_queue.join()
        '''
        self.search_message_queue.join()
