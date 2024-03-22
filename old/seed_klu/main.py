# ******************************************************
# Made by Krzysztof Łukasiewicz
# 23.02.2023 03:55
# ******************************************************
from engine import Engine
import logging




if __name__ == '__main__':
    logging.basicConfig(format='%(asctime)s; %(process)d-%(levelname)s: %(message)s')
    my_engine = Engine()
    my_engine.engine_loop()