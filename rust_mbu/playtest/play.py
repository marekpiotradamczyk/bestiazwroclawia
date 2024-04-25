import subprocess
import re
import time
import argparse

argparser = argparse.ArgumentParser("Benchmark Morphebot against Stockfish")

argparser.add_argument(
    "--rounds",
    type=int,
    default=50,
    help="Number of games to play for each elo rating and side, by default 50",
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
    default="1+0",
    help="Time control for the games, by default 1+0",
)
argparser.add_argument(
    "--concurrency",
    type=int,
    default=5,
    help="Number of games to play at the same time, by default 5",
)
argparser.add_argument(
    "--label",
    type=str,
    default=None,
    help="Label for the run, by default its the current time",
)

argparser.add_argument(
    "--errors", action="store_true", help="Print Morphebot error messages"
)

args = argparser.parse_args()
print(args.errors)


def setup_game_vs_stockfish(elo, rounds, time_control, play_as_white=True):
    general = [
        "./c-chess-cli/c-chess-cli",
        "-each",
        f"tc={time_control}",
        "option.Threads=1",
    ]
    stockfish = [
        "-engine",
        "cmd=stockfish",
        f"name=stockfish {elo}",
        f"option.UCI_Elo={elo}",
        "option.UCI_LimitStrength=true",
    ]
    morphebot = ["-engine", "cmd=morphebot", "name=morphebot"]
    rest = ["-rounds", str(rounds), "-concurrency", str(args.concurrency)]

    stderr = subprocess.DEVNULL if not args.errors else None

    if play_as_white:
        out = subprocess.check_output(
            general + morphebot + stockfish + rest, stderr=stderr
        ).decode("utf-8")

        handle_output(out, elo, True)
    else:
        out = subprocess.check_output(
            general + stockfish + morphebot + rest, stderr=stderr
        ).decode("utf-8")

        handle_output(out, elo, False)


def handle_output(output, elo, play_as_white=True):
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
    print(f"Win ratio for elo {elo} ({color}): {winratio:.2f}%")
    label = (
        time.strftime("%Y-%m-%d %H:%M:%S", time.localtime())
        if args.label is None
        else args.label
    )
    with open("results.csv", "a") as f:
        if f.tell() == 0:
            f.write("Label,Elo,Time Control,Color,Wins,Draws,Loses,Winratio\n")
        f.write(
            f"{label},{elo},{args.time_control},{color},{wins},{
                draws},{loses},{winratio:.2f}%\n"
        )


def play():
    for elo in range(args.start_elo, args.end_elo + 1, args.elo_step):
        print("Playing games against stockfish elo: ", elo)
        setup_game_vs_stockfish(elo, args.rounds, args.time_control)
        setup_game_vs_stockfish(elo, args.rounds, args.time_control, False)


play()
