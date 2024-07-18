import re
import pandas as pd
import plotly.express as px
from collections import namedtuple
from plotly.subplots import make_subplots
import plotly.graph_objects as go
import chess.pgn
import io

PATHS = ["morphe_nn.out", "morphe_random.out"]
BOTNAMES = ["morphebot_nn", "morphebot_random"] 

DepthInfo = namedtuple('Depth_info', ['inc', 'nodes'])

class PGNProvider:
    
    def __init__(self, path : str, skip : int = 0):
        c = 0
        for line in open(path, "r"):
            if line == '\n':
                c += 1
        
        self.len = (c + 1) // 2
        self.read_count = 0
        self.pgn_file = open(path, "r")

        if skip >= self.len:
            raise Exception("Not enough games to skip")
        
        for _ in range(skip):
            while self.pgn_file.readline() != '\n': pass
            while self.pgn_file.readline() != '\n': pass
            self.len -= 1
    
    def __iter__(self):
        return self
    
    def __len__(self):
        return self.len
    
    def __next__(self):
        pgn_string = ""
        c = False

        while True:
            line = self.pgn_file.readline()
            
            if line == '':
                raise StopIteration
            
            if (c and line == '\n') or line == '':
                break
            elif line == '\n':
                c = True
            pgn_string += line   
        self.read_count += 1
        return pgn_string
    
    def __str__(self):
        return f"Read positions : {self.read_count} Limit : {self.limit} PGN file : {self.pgn_file}"


class GameInfo:

    def __init__(self, side, board, fen, n):
        self.moves = 0
        self.move_history = {}
        self.side = side
        self.side_str = "WHITE" if chess.WHITE == side else "BLACK"
        self.board = board
        self.fen = fen
        self.n = n
    
    def add_move(self, inc):
        self.moves += 1
        self.move_history[self.moves] = DepthInfo(inc, [])

    def add_nodes(self, nodes):
        self.move_history[self.moves].nodes.append(nodes)

    def avg_inc(self):
        avg = 0
        for v in self.move_history.values():
            avg += v.inc
        return avg / self.moves

    def avg_nodes(self):
        avg = 0 

        for v in self.move_history.values():
            avg += v.nodes[-1]
        
        return avg / self.moves

    def sum_nodes(self):
        nodes = 0

        for v in self.move_history.values():
            nodes += v.nodes[-1]
        return nodes

    def compute_dist(self):
        nodes = self.sum_nodes()
        dist = {'moves' : [], 'relative' : [], 'absolute' : [], 'inc' : []}

        for i, v in enumerate(self.move_history.values()):
            dist['moves'].append(i)
            dist['relative'].append(sum(v.nodes) / nodes)
            dist['absolute'].append(sum(v.nodes))
            dist['inc'].append(v.inc)
        
        return dist
    
    def get_increases(self):
        inc = []

        for v in self.move_history.values():
            inc.append(v.inc)
        return inc

    def is_winner(self):
        outcome = self.board.outcome(claim_draw=True)

        if outcome is None:
            return False

        return True if self.side == outcome.winner else False


    def get_game_plot(self):
        dist = self.compute_dist()
        df = pd.DataFrame(dist)
        return go.Bar(
                x=df['moves'], y=df['absolute'],
                marker=dict(colorscale="redor"),
                marker_color=df['inc']), max(dist['absolute'])

    def get_plot_title(self):
        outcome = self.board.outcome(claim_draw=True)
        result = "" if outcome is None else outcome.result()
        termination = "Rules infraction" if outcome is None else outcome.termination
        return f"""{self.n}. {self.side_str}, {result}, {termination}, winner: {self.is_winner()}"""

    
    def __str__(self):
        return f"""Moves: {self.moves}, 
        Avg inc: {self.avg_inc()}, 
        Avg nodes: {self.avg_nodes()},
        Outcome: {self.board.outcome(claim_draw=True)},
        {self.move_history}"""


def parse_games(path, botname):
    games = []
    game_info = GameInfo(None, None, None, None)
    pgn_provider = PGNProvider('../games.pgn')
    game = None
    n = 0
    for line in open(path):
        line = line.rstrip()
        if "ucinewgame" in line:
            n += 1
            if len(game_info.move_history) > 0:
                games.append(game_info)

            game = chess.pgn.read_game(io.StringIO(next(pgn_provider)))
            board = game.end().board()
            fen = game.board().fen()

            if game.headers['White'] == botname:
                game_info = GameInfo(chess.WHITE, board, fen, n)
            else: 
                game_info = GameInfo(chess.BLACK, board, fen, n)


        elif "depth_debug" in line:
            depth_increase = int(line[-1])
            game_info.add_move(depth_increase)
        
        elif "info" in line:
            x = re.search("nodes (\\d+)", line)
            nodes = x.group(1) 
            game_info.add_nodes(int(nodes))
    games.append(game_info)
    return games


def countup_avg(games):
    ainc = 0
    anodes = 0
    rdist = [0 for i in range(120)]
    adist = [0 for i in range(120)]


    for game in games:
        ainc += game.avg_inc()
        anodes += game.avg_nodes()

        dist = game.compute_dist()

        r = dist['relative']
        a = dist['absolute']
        for i, v in enumerate(r):
            if i >= 120:
                break
            rdist[i] += v
        
        for i, v in enumerate(a):
            if i >= 120:
                break
            adist[i] += v

    games_len = len(games)
    return (ainc / games_len, anodes / games_len, 
    [x / games_len for x in rdist],
    [x / games_len for x in adist])


def add_games(games):
    result = [0 for i in range(256)]
    for game in games:
        for i in range(len(game)):
            result[i] += game[i]
    return result


games_nn = parse_games(PATHS[0], BOTNAMES[0])
added_nn = countup_avg(games_nn)
print(f"{PATHS[0]}  mean inc: {added_nn[0]}, mean nodes: {added_nn[1]}")
    
games_random = parse_games(PATHS[1], BOTNAMES[1])
added_random = countup_avg(games_random)
print(f"{PATHS[1]}  mean inc: {added_random[0]}, mean nodes: {added_random[1]}")

df = pd.DataFrame({'move' : list(range(max(len(added_nn[2]), len(added_random[2])))),
    'nn_relative' : added_nn[2], 'random_relative' : added_random[2], 
    'nn_absolute' : added_nn[3], 'random_absolute' : added_random[3]})

# fig = px.bar(df, x='move', y=['nn_relative', 'random_relative'],
#     color_continuous_scale='redor', barmode='group',
#     title=f"Average % of nodes calculated in game for move")
# fig.show()

# fig = px.bar(df, x='move', y=['nn_absolute', 'random_absolute'],
#     color_continuous_scale='redor', barmode='group',
#     title=f"Average number of nodes calculated in game for move")
# fig.show()

print(len(games_nn))

def plot_single_game(start, end):
    
    fig = make_subplots(rows=end-start, cols=2, 
        subplot_titles=['asd' for _ in range(2*(end-start))]
    )
    k = start
    for j in range(start, end):
        idx = k - start + 1

        nn_game = games_nn[j]
        random_game = games_random[j]

        #if not (not nn_game.is_winner() and random_game.is_winner()):
        #    continue

        nn_plot, nn_max_node = nn_game.get_game_plot()
        random_plot, random_max_node = random_game.get_game_plot()

        nn_title = nn_game.get_plot_title()
        random_title = random_game.get_plot_title()

        max_nodes = max(nn_max_node, random_max_node)

        fig.add_trace(nn_plot, row=idx, col=1)
        fig.add_trace(random_plot, row=idx, col=2)

        fig.update_layout(
            {
                f'yaxis{2*idx-1}': {'range' : [0, max_nodes]},
                f'yaxis{2*idx}': {'range' : [0, max_nodes]}
            }
        )
        fig.layout.annotations[2*(idx-1)].update(text=nn_title)
        fig.layout.annotations[2*idx-1].update(text=random_title)

        k += 1

    fig.update_layout(
        title_text="Computed nodes and increases per move",
        height=1000
    )

    fig.show()


plot_single_game(0, 2)
