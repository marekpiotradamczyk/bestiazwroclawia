from chess.pgn import read_game
from chess.engine import PovScore
from chess.pgn import Game

MAX_SCORE = 3000


def collect_data(input, output):
    input_file = open(input, "r")
    output_file = open(output, "a+")
    with open("last_open.txt", "r") as last_open:
        line = last_open.readline()
        if line != "":
            last = int(last_open.readline())
            input_file.seek(last)

    visited_games = {}

    output_file.seek(0)
    for entry in output_file.readlines():
        fen = entry.split(",")[0]
        visited_games[fen] = 1
    output_file.seek(0, 2)

    cnt = len(visited_games)

    for game in read_games(input_file):
        for fen, eval in game:
            if fen not in visited_games:
                output_file.write(f"{fen},{eval}\n")
                visited_games[fen] = eval
                cnt += 1
                if cnt % 1000 == 0:

                    with open("last_open.txt", "w") as last_open:
                        last_open.write(f"{input_file.tell()}\n")
                    print(f"Processed {cnt} positions")


def read_games(input_file):
    while True:
        game = read_game(input_file)

        yield process_game(game)

        if game is None:
            break


def process_game(game: Game):
    curr_pos = game.root().next()
    while True:
        if curr_pos is None:
            break
        if curr_pos.eval() is None:
            break

        eval = normalize_eval(curr_pos.eval())
        yield (curr_pos.board().fen(), eval)
        curr_pos = curr_pos.next()


def normalize_eval(eval: PovScore):
    mate = eval.relative.mate()
    score = None
    if mate is not None:
        if mate > 0:
            score = MAX_SCORE
        else:
            score = -MAX_SCORE

    if score is None:
        score = eval.relative.score()

    return (score + MAX_SCORE) / (2 * MAX_SCORE)
