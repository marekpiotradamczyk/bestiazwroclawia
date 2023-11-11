use std::time::Instant;

use derivative::Derivative;

#[derive(Debug)]
pub struct TimeControl {
    start_time: Instant,
    limit: Option<isize>,
    is_infinite: bool,
}

impl TimeControl {
    pub fn search_time(&self, now: Instant) -> isize {
        now.duration_since(self.start_time).as_millis() as isize
    }

    pub fn remaining_time(&self, now: Instant) -> isize {
        if self.is_infinite {
            return isize::MAX;
        }
        self.limit.unwrap_or_default() - self.search_time(now)
    }

    pub fn is_over(&self) -> bool {
        if self.is_infinite {
            return false;
        }

        self.remaining_time(Instant::now()) <= 0
    }
}

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct SearchOptions {
    pub depth: Option<usize>,
    pub nodes: Option<usize>,
    pub movetime: Option<isize>,
    pub infinite: bool,
    pub ponder: bool,
    pub wtime: Option<isize>,
    pub btime: Option<isize>,
    pub winc: Option<isize>,
    pub binc: Option<isize>,
    pub movestogo: Option<isize>,
}

impl SearchOptions {
    pub fn time_control(&self, is_white: bool) -> TimeControl {
        if self.infinite {
            return TimeControl {
                start_time: Instant::now(),
                limit: None,
                is_infinite: true,
            };
        }

        if self.movetime.is_some() {
            return TimeControl {
                start_time: Instant::now(),
                limit: self.movetime,
                is_infinite: false,
            };
        }

        if (is_white && self.wtime.is_none()) || (!is_white && self.btime.is_none()) {
            return TimeControl {
                start_time: Instant::now(),
                limit: None,
                is_infinite: true,
            };
        }

        let _increment = if is_white {
            self.winc.unwrap_or(0)
        } else {
            self.binc.unwrap_or(0)
        };

        let time_left = if is_white {
            self.wtime.unwrap_or(0)
        } else {
            self.btime.unwrap_or(0)
        };

        let moves_to_go = self.movestogo.unwrap_or(40).max(1);

        let limit = time_left / moves_to_go;

        if limit <= 0 {
            return TimeControl {
                start_time: Instant::now(),
                limit: None,
                is_infinite: false,
            };
        }

        TimeControl {
            start_time: Instant::now(),
            limit: Some(limit),
            is_infinite: false,
        }
    }
}
