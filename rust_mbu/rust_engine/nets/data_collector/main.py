import argparse
from data_collector import collect_data

parser = argparse.ArgumentParser(
    prog="Lichess dataset collector",
)

parser.add_argument("--output", help="Output file", default="dataset.csv")

parser.add_argument("--input", help="Input file", default="dataset.pgn")

args = parser.parse_args()

collect_data(args.input, args.output)
