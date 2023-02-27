#!path/to/interpreter
# ******************************************************
# Made by Krzysztof Łukasiewicz
# 26.02.2023 23:20
# ******************************************************
from engine import Engine
import logging




if __name__ == '__main__':
    logging.basicConfig(level=logging.DEBUG, filename="log.txt", filemode="w",format='%(asctime)s; %(process)d-%(levelname)s: %(message)s')
    my_engine = Engine()
    my_engine.engine_loop()