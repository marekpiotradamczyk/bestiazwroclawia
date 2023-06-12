from threading import Thread
from queue import Queue
import commands_data as CD

def _is_command(command : str) -> bool:
    return command in ['uci', 'debug', 'isready', 'setoption', 'ucinewgame', 'position', 'go', 'stop', 'ponderhit', 'quit']

def _handle_input(line : str) -> tuple[str, str]:
    where_whitespace = line.find(' ')
    if where_whitespace == -1:
        return [line, '']
    word = line[:where_whitespace]
    if line != '' and _is_command(word):
        return [word, line[where_whitespace:].strip()]
    return ['', '']

# Honestly, I considered using deque, instead of two queues, but decided against it.
# There would be big confusion about messages otherwise, because
# input handles one command at a time, but output can handle more than one.
class UCI_Input(Thread):
    def __init__(self, author : str, name : str, input_message_queue : Queue, output_message_queue : Queue) -> None:
        super().__init__()
        self.author = author
        self.name = name
        self.input_message_queue = input_message_queue
        self.output_message_queue = output_message_queue

    def run(self):
        self._identification()

        while True:
            try:
                line = input().strip()
            except EOFError:
                break

            command, args = _handle_input(line)
            if command == '':
                continue
                # self.message_queue.task_done()
            elif command == 'quit':
                break
            getattr(self, '_'+command+'_command')(args)
        self._quit_command()

    def _identification(self):
        self.output_message_queue.put(['id', CD.IdCommand(name=self.name, author=self.author)])
        self.output_message_queue.put(['uciok', None])

    def _uci_command(self, args : str):
        # I belive 'pass' is correct response for this command
        # because in during the game there is no place or time for it,
        # it's simply unnecessary.
        pass

    def _debug_command(self, args : str):
        pass

    def _isready_command(self, args : str):
        self.output_message_queue.put(['readyok', None])

    def _setoption_command(self, args : str):
        pass

    def _ucinewgame_command(self, args : str):
        pass

    def _position_command(self, args : str):
        fen = args
        moves = list()
        if 'moves' in args:
            i = args.index('moves')
            fen = args[:i] # TO BE CHANGED!
            moves = args[i+5:].split()
        
        if fen == 'startpos':
            self.input_message_queue.put(['position', CD.PositionCommand(moves=moves)])
        else:
            self.input_message_queue.put(['position', CD.PositionCommand(fen=fen, moves=moves)])

    def _go_command(self, args : str):
        is_subcommand = lambda x: x in ['searchmoves', 'ponder', 'wtime', 'btime', 'winc', 'binc', 'movestogo',
                       'depth', 'nodes', 'mate', 'movetime', 'infinite']
        
        def is_valid_move(m : str) -> bool:
            if m == '0000':
                return True
            if len(m) in [4, 5]:
                res = all(x>=ord('1') and x<=ord('8') for x in [ord(m[0]), ord(m[2])])
                res &= all(x>=ord('a') and x<=ord('h') for x in [ord(m[1]), ord(m[3])])
                if len(m) == 5:
                    res &= m[4] in ['r', 'n', 'b', 'q']
                return res
            return False
        
        arguments = args.split()
        l = len(arguments)
        command_to_send = CD.GoCommand()
        for i in range(l):
            x = arguments[i]
            if is_subcommand(x):
                if x == 'searchmoves':
                    command_to_send.searchmoves = []
                    for j in range(i + 1, l):
                        y = arguments[j]
                        if is_subcommand(y):
                            i = j
                            break
                        if is_valid_move(y):
                            command_to_send.searchmoves.append(y)
                elif x == 'ponder':
                    command_to_send.ponder = True
                elif x == 'infinite':
                    command_to_send.infinite = True
                elif i+1<l and arguments[i+1].isnumeric():
                    setattr(command_to_send, x, int(arguments[i+1]))
        self.input_message_queue.put(['go', command_to_send])

    def _stop_command(self, args : str):
        self.input_message_queue.put(['stop', None])

    def _ponderhit_command(self, args : str):
        pass
        
    def _quit_command(self):
        self.input_message_queue.put(['quit', None])
        self.output_message_queue.put(['quit', None])