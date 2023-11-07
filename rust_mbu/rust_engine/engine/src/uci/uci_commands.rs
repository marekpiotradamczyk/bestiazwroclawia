use std::{str::FromStr, error::Error};

use sdk::position::Position;

use crate::engine::search::utils::time_control::SearchOptions;

pub enum Command {
    Uci,
    Position(Position, Vec<String>),
    SetOption(String, Option<String>),
    Go(SearchOptions),
    Stop,
    UciNewGame,
    IsReady,
    Quit,
}


impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stop" => Ok(Command::Stop),
            "isready" => Ok(Command::IsReady),
            "uci" => Ok(Command::Uci),
            "quit" => Ok(Command::Quit),
            "ucinewgame" => Ok(Command::UciNewGame),
            _ => Err("Invalid command".into()),
        }
    }
}

