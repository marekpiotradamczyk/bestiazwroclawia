from threading import Thread
from queue import Queue
import commands_data as CD
import alfa


class Search(Thread):
    def __init__(self, message_queue: Queue):
        self.message_queue = message_queue

    def run(self):  # 'quit', 'searchposition', 'stop' [str, obj]
        while True:
            if not self.message_queue.empty():
                message, args = self.message_queue.get()
                match message:
                    case 'quit':
                        break
                    case 'SearchPosition':
                        return self._handle_searchposition_command(args)
                    case 'stop':
                        self._handle_stopsearch_command()

    def _handle_searchposition_command(self, command: CD.StartSearchCommand):
        self.position = command.board
        print("DO SEARCH")
        print(self.position)
        BestMove = alfa.Search(self.position, command.depth, command.time)
        print(f"DONE: {BestMove}")
        BMC = CD.BestMoveCommand()
        BMC.bestmove = BestMove
        return (["bestmove", BMC])

    def _handle_stopsearch_command(self):
        # self.message_queue.put([str, obj])
        alfa.Stop()
        # self.connection.send(BestMoveCommand(move=self.best_move))

    def _QuitCommand():
        print("ELO")
