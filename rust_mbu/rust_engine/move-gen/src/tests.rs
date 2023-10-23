#![allow(dead_code)]
use std::{collections::HashSet, thread};

use sdk::{
    fen::Fen,
    position::{Color, Position},
};
use serde::Deserialize;

use crate::{
    generators::movegen::MoveGen,
    utils::{chess_notation::ChessNotation, logger::configure_logger},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Test {
    #[serde(default)]
    description: String,
    test_cases: Vec<TestCase>,
}

#[derive(Deserialize, Debug)]
struct TestCase {
    start: StartPosition,
    expected: Vec<MoveFen>,
}

#[derive(Deserialize, Debug)]
struct StartPosition {
    #[serde(default)]
    description: String,
    fen: String,
}

#[derive(Deserialize, Debug)]
struct MoveFen {
    r#move: String,
    fen: String,
}

fn load_test(file_name: String) -> Test {
    let home = env!("CARGO_MANIFEST_DIR");
    let test = std::fs::read_to_string(format!("{home}/src/test_cases/{file_name}")).unwrap();

    serde_json::from_str(&test).unwrap()
}

#[test]
fn test_all() {
    configure_logger();
    info!("Starting tests");

    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(run_all_tests)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn run_all_tests() {
    let test_dir = std::fs::read_dir("src/test_cases").unwrap();

    for file in test_dir {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        info!("Running tests for {}", file_name);
        run_test(file_name);
    }
}

fn run_test(json_name: String) {
    let move_gen = MoveGen::new();
    let test_cases = load_test(json_name.clone());

    for (idx, test_case) in test_cases.test_cases.iter().enumerate() {
        let pos = Position::from_fen(test_case.start.fen.clone()).unwrap();

        let expected_moves: HashSet<String> = test_case
            .expected
            .iter()
            .map(|expected| expected.r#move.clone())
            .collect();

        let actual_moves: HashSet<String> = move_gen
            .generate_legal_moves(&pos)
            .map(|mv| move_gen.to_algebraic_notation(&pos, &mv))
            .collect();

        let expected_not_actual: HashSet<String> =
            expected_moves.difference(&actual_moves).cloned().collect();

        let actual_not_expected: HashSet<String> =
            actual_moves.difference(&expected_moves).cloned().collect();

        assert!(
            expected_not_actual.is_empty(),
            "{pos}\nActual: {:?}\nExpected not actual: {:?}\nDescription: {}\nFen: {}",
            actual_moves,
            expected_not_actual,
            test_case.start.description,
            test_case.start.fen
        );
        assert!(
            actual_not_expected.is_empty(),
            "{pos}\nActual: {:?}\nActual not expected: {:?}\nDescription: {}\nFen: {}",
            actual_moves,
            actual_not_expected,
            test_case.start.description,
            test_case.start.fen
        );
        info!("[{} ({})] passed.", json_name, idx + 1);
    }
}
