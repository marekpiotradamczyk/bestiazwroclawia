from threading import Thread
from queue import Queue
import commands_data as CD
import alfa


class Search(Thread):
    def __init__(self, input_message_queue: Queue, output_message_queue: Queue):
        super().__init__()
        self.input_message_queue = input_message_queue
        self.output_message_queue = output_message_queue
        self.bestmove = None
        # It's really not necessary here
        # self.board

    def run(self):
        while True:
            if not self.input_message_queue.empty():
                message, args = self.input_message_queue.get()
                match message:
                    case 'quit':
                        break
                    case 'searchposition':
                        self._searchposition_command(args)
                    case 'stop':
                        self.stopsearch_command()

    def _searchposition_command(self, settings: CD.StartSearchCommand):
        self.bestmove = alfa.Search(settings.board, settings.depth, settings.movetime, settings.posHash)
        self.output_message_queue.put(['bestmove', CD.BestMoveCommand(bestmove=self.bestmove)])

    def stopsearch_command(self):
        alfa.Stop()
        # self.output_message_queue.put(['bestmove', CD.BestMoveCommand(bestmove=self.bestmove)])
