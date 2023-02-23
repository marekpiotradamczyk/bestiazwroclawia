import time



class TimeManager(object):
    def __init__(self):
        self.infinite = False
        self.timer = 0.0
        self.start_time = 0

    def set_timer(self, t : int, infinite : bool):
        self.infinite = infinite
        self.start_time = time.perf_counter()
        self.timer = float(t)/1000.0

    def out_of_time(self):
        curr_time = time.perf_counter()
        return not self.infinite and curr_time-self.start_time > self.timer