import subprocess
import re
import time
import argparse
from copy import deepcopy

argparser = argparse.ArgumentParser("Benchmark Morphebot against Stockfish")

argparser.add_argument(
    "--rounds",
    type=int,
    default=1,
    help="Number of repetition of games to play for each elo rating and side, by default 1",
)
argparser.add_argument(
    "--games",
    type=int,
    default=50,
    help="Number of games to play"
)
argparser.add_argument(
    "--start_elo", type=int, default=1400, help="Start elo rating, by default 1400"
)
argparser.add_argument(
    "--end_elo", type=int, default=2500, help="End elo rating, by default 2500"
)
argparser.add_argument(
    "--elo_step", type=int, default=100, help="Step between elo ratings, by default 100"
)
argparser.add_argument(
    "--time_control",
    type=str,
    default="5+0",
    help="Time control for the games, by default 5+0",
)
argparser.add_argument(
    "--concurrency",
    type=int,
    default=3,
    help="Number of games to play at the same time, by default 3",
)
argparser.add_argument(
    "--label",
    type=str,
    default=None,
    help="Label for the run, by default its the current time",
)
argparser.add_argument(
    "--engine1",
    nargs='+',
    type=str,
    default=["./morphebot", "name=morphebot"],
    help="Engine that plays against engine2"
)
argparser.add_argument(
    "--engine2",
    nargs='+',
    type=str,
    default=["./stockfish", "name=stockfish `elo`", "option.UCI_Elo=`elo`", 
             "option.UCI_LimitStrength=true"],
    help="Engine that plays against engine1"
)
argparser.add_argument(
    "--openings",
    type=str,
    default=None,
)

argparser.add_argument(
    "--errors", action="store_true", help="Print Morphebot error messages"
)

args = argparser.parse_args()

def setup_game(args, first_white=True):
    rounds = args.rounds
    time_control = args.time_control
    general = [
        "./c-chess-cli/c-chess-cli",
        "-each",
        f"tc={time_control}",
        "option.Threads=1",
    ]

    engine1 = ["-engine",f"cmd={args.engine1[0]}",*args.engine1[1:]]
    engine2 = ["-engine", f"cmd={args.engine2[0]}", *args.engine2[1:]]

    rest = ["-games", str(args.games), 
            "-rounds", str(rounds), 
            "-concurrency", str(args.concurrency),
            "-log", 
            "-pgn", "games.pgn", "1",]

    if args.openings != None:
        rest += ["-openings", f"file={args.openings}", "-repeat"]

    stderr = subprocess.DEVNULL if not args.errors else None

    if first_white:
        out = subprocess.check_output(
            general + engine2 + engine1 + rest, stderr=stderr
        ).decode("utf-8")

        handle_output(out, True)
    else:
        out = subprocess.check_output(
            general + engine1 + engine2 + rest, stderr=stderr
        ).decode("utf-8")

        handle_output(out, False)


def handle_output(output, play_as_white=True):
    last = output.split("\n")[-2]
    parsed = re.search(r"(\d*) - (\d*) - (\d*)", last)
    if play_as_white:
        wins = int(parsed.group(1))
        loses = int(parsed.group(2))
        draws = int(parsed.group(3))
    else:
        wins = int(parsed.group(2))
        loses = int(parsed.group(1))
        draws = int(parsed.group(3))

    winratio = (wins + draws / 2) / (wins + loses + draws) * 100
    color = "White" if play_as_white else "Black"
    print(output)
    print(f"Win ratio: {winratio:.2f}%")
    label = (
        time.strftime("%Y-%m-%d %H:%M:%S", time.localtime())
        if args.label is None
        else args.label
    )
    with open("results.csv", "a") as f:
        if f.tell() == 0:
            f.write("Label,Elo,Time Control,Color,Wins,Draws,Loses,Winratio\n")
        f.write(
            f"{label},{args.time_control},{color},{wins},{
                draws},{loses},{winratio:.2f}%\n"
        )


def play():
    

    for elo in range(args.start_elo, args.end_elo + 1, args.elo_step):
        arg = deepcopy(args)
        arg.engine1 = list(map(lambda x: x.replace("`elo`", str(elo)), args.engine1))
        arg.engine2 = list(map(lambda x: x.replace("`elo`", str(elo)), args.engine2))
        engine1_name = f"{next(filter(lambda x: x.find('name') > -1, arg.engine1)).replace('name=', '')}"
        engine2_name = f"{next(filter(lambda x: x.find('name') > -1, arg.engine2)).replace('name=', '')}"

        print(f"{engine1_name} playing against {engine2_name}:")
        setup_game(arg)
     #   setup_game(arg, False)


play()
