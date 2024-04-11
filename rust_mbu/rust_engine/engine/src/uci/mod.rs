pub mod commands;

use std::io;
use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;
use sdk::{fen::Fen, position::Position};

use crate::engine::search::utils::time_control::SearchOptions;
use crate::engine::Engine;
use crate::uci::commands::Command;

pub type Result<T> = anyhow::Result<T>;

/// # Panics
/// Panics if the command is invalid
pub fn start() {
    let tx = Engine::start_loop_thread();

    println!("ready");

    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line");

        let split = line.split_whitespace().collect_vec();

        let (command, args) = split
            .split_first()
            .map_or((&"", vec![]), |(cmd, args)| (cmd, args.to_vec()));

        let command = match command.to_lowercase().as_str() {
            "position" => parse_position(&args),
            "go" => parse_go(&args),
            "setoption" => parse_set_option(&args),
            "simulate" => Ok(Command::Simulate(
                args.into_iter().map(ToString::to_string).collect_vec(),
            )),
            "quit" => return,
            any => Command::from_str(any).map_err(|_| anyhow!("Unknown command {any}")),
        };
        match command {
            Ok(command) => tx.send(command).expect("Failed to send command"),
            Err(e) => println!("{e}"),
        }
    }
}

fn parse_position(args: &[&str]) -> Result<Command> {
    if args.is_empty() {
        return Err(anyhow!("Missing FEN or startpos"));
    }

    let (pos, idx) = if args[0] == "startpos" {
        (Position::default(), 1)
    } else {
        let mut iter = args.iter().skip(1).take_while(|s| **s != "moves");
        let idx = iter.clone().count() + 1;
        let fen = iter.join(" ");

        let pos = Position::from_fen(fen)?;

        (pos, idx)
    };

    let moves = args.iter().skip(idx).map(ToString::to_string).collect_vec();

    let moves = if moves.is_empty() {
        vec![]
    } else if moves[0] == "moves" {
        moves[1..].to_vec()
    } else {
        return Err(anyhow!("Expected 'moves'"));
    };

    Ok(Command::Position(pos, moves))
}

fn parse_go(args: &[&str]) -> Result<Command> {
    let mut idx = 0;
    let mut search_options = SearchOptions::default();

    // Parse search options
    while idx < args.len() {
        let token = args[idx];

        macro_rules! parse_set_field {
            ($field:ident) => {{
                let value = args
                    .get(idx + 1)
                    .ok_or(anyhow!("Missing value for {token}"))?
                    .parse()?;

                search_options.$field = Some(value);
                idx += 2;
            }};
        }

        macro_rules! parse_flag_field {
            ($field:ident) => {{
                search_options.$field = true;
                idx += 1;
                continue;
            }};
        }

        match token {
            "depth" => parse_set_field!(depth),
            "nodes" => parse_set_field!(nodes),
            "movetime" => parse_set_field!(movetime),
            "infinite" => parse_flag_field!(infinite),
            "ponder" => parse_flag_field!(ponder),
            "wtime" => parse_set_field!(wtime),
            "btime" => parse_set_field!(btime),
            "winc" => parse_set_field!(winc),
            "binc" => parse_set_field!(binc),
            "movestogo" => parse_set_field!(movestogo),
            _ => return Err(anyhow!("Unknown token {token}")),
        }
    }

    if search_options.depth.is_some() {
        search_options.infinite = false;
    }

    Ok(Command::Go(search_options))
}

/// # Panics
/// Panics if the command is invalid
/// # Errors
/// Returns an error if the command is invalid
pub fn parse_set_option(args: &[&str]) -> Result<Command> {
    if args.len() < 2 {
        return Err(anyhow!("Missing option name"));
    }

    if args[0] != "name" {
        return Err(anyhow!("Expected 'name'"));
    }

    let value_idx = args.iter().take_while(|s| **s != "value").count();

    let name = args[1..value_idx].join(" ");
    let value = args.get(value_idx + 1).map(ToString::to_string);

    Ok(Command::SetOption(name, value))
}
