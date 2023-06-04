from threading import Thread
from queue import Queue
import commands_data as CD

class UCI_Output(Thread):
    def __init__(self, message_queue : Queue) -> None:
        super().__init__()
        self.message_queue = message_queue

    def run(self):
        while True:
            command, args = self.message_queue.get()
            if command == 'quit':
                break
            getattr(self, '_'+command+'_command')(args)
            self.message_queue.task_done()
        self.message_queue.task_done()
    
    def _id_command(self, args : CD.IdCommand):
        print(f"name {args.name}", flush=True)
        print(f"author {args.author}", flush=True)

    def _uciok_command(self, args : None):
        print('uciok', flush=True)

    def _readyok_command(self, args : None):
        print("readyok", flush=True)

    def _bestmove_command(self, args : CD.BestMoveCommand):
        print(f"bestmove {args.bestmove} ponder ...", flush=True)

    def _info_command(self, args : CD.InfoCommand):
        pass
