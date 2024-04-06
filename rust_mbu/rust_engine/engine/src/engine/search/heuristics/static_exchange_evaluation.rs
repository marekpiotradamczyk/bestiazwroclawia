use move_gen::{
    generators::{pieces::simple_move_generator::SimpleMoveGenerator},
    r#move::Move,
};
use sdk::{
    bitboard::Bitboard,
    fen::Fen,
    position::{Color, Piece, Position},
    square::Square,
};

use crate::engine::{eval::PIECE_VALUES, MOVE_GEN};

pub fn static_exchange_evaluation(pos: &Position, mv: &Move) -> i32 {
    let mut gain = [0; 32];
    let mut occupied = pos.occupied;
    let bishop_sliders = pos.pieces[Color::White as usize][Piece::Bishop as usize]
        | pos.pieces[Color::Black as usize][Piece::Bishop as usize]
        | pos.pieces[Color::White as usize][Piece::Queen as usize]
        | pos.pieces[Color::Black as usize][Piece::Queen as usize];

    let rook_sliders = pos.pieces[Color::White as usize][Piece::Rook as usize]
        | pos.pieces[Color::Black as usize][Piece::Rook as usize]
        | pos.pieces[Color::White as usize][Piece::Queen as usize]
        | pos.pieces[Color::Black as usize][Piece::Queen as usize];

    let target_sq = mv.to();
    let from_sq = mv.from();
    let mut turn = pos.turn;

    let mut attacks = MOVE_GEN.attacks_to_square(pos, target_sq, Color::White, occupied)
        | MOVE_GEN.attacks_to_square(pos, target_sq, Color::Black, occupied);

    let mut attacked_piece_val = PIECE_VALUES[pos.piece_at(&target_sq).unwrap().0 as usize];
    turn = turn.enemy();

    gain[0] = attacked_piece_val;

    let piece = pos.piece_at(&from_sq).unwrap().0;
    occupied ^= from_sq.bitboard();

    attacked_piece_val = PIECE_VALUES[piece as usize];

    if matches!(piece, Piece::Pawn | Piece::Bishop | Piece::Queen) {
        attacks |= MOVE_GEN.bishop_moves(target_sq, occupied) & bishop_sliders;
    }

    if matches!(piece, Piece::Rook | Piece::Queen) {
        attacks |= MOVE_GEN.rook_moves(target_sq, occupied) & rook_sliders;
    }

    let mut counter = 0;
    let mut remaining_attackers = attacks & occupied;
    while !remaining_attackers.is_empty() {
        let least_valuable_piece_sq =
            if let Some(sq) = least_valuable_piece(pos, remaining_attackers, turn) {
                sq
            } else {
                break;
            };
        let piece = pos.piece_at(&least_valuable_piece_sq).unwrap().0;

        occupied ^= least_valuable_piece_sq.bitboard();

        if matches!(piece, Piece::Pawn | Piece::Bishop | Piece::Queen) {
            remaining_attackers |= MOVE_GEN.bishop_moves(target_sq, occupied) & bishop_sliders;
        }

        if matches!(piece, Piece::Rook | Piece::Queen) {
            remaining_attackers |= MOVE_GEN.rook_moves(target_sq, occupied) & rook_sliders;
        }

        counter += 1;
        if counter == 32 {
            println!("{pos}");
            println!("{mv}");
            println!("{}", pos.to_fen());
        }
        gain[counter] = attacked_piece_val - gain[counter - 1];
        attacked_piece_val = PIECE_VALUES[piece as usize];
        if gain[counter] > attacked_piece_val {
            break;
        }
        turn = turn.enemy();
        remaining_attackers = attacks & occupied;
    }

    while counter > 0 {
        gain[counter - 1] = -i32::max(-gain[counter - 1], gain[counter]);
        counter -= 1;
    }

    gain[0]
}

pub fn static_exchange_evaluation_move_done(pos: &Position, mv: &Move) -> i32 {
    let mut gain = [0; 32];
    let mut occupied = pos.occupied;
    let bishop_sliders = pos.pieces[Color::White as usize][Piece::Bishop as usize]
        | pos.pieces[Color::Black as usize][Piece::Bishop as usize]
        | pos.pieces[Color::White as usize][Piece::Queen as usize]
        | pos.pieces[Color::Black as usize][Piece::Queen as usize];

    let rook_sliders = pos.pieces[Color::White as usize][Piece::Rook as usize]
        | pos.pieces[Color::Black as usize][Piece::Rook as usize]
        | pos.pieces[Color::White as usize][Piece::Queen as usize]
        | pos.pieces[Color::Black as usize][Piece::Queen as usize];

    let target_sq = mv.to();
    let from_sq = mv.from();
    let mut turn = pos.turn;

    let piece = pos.piece_at(&from_sq).unwrap().0;

    let mut attacks = MOVE_GEN.attacks_to_square(pos, target_sq, Color::White, occupied)
        | MOVE_GEN.attacks_to_square(pos, target_sq, Color::Black, occupied);

    let mut attacked_piece_val = PIECE_VALUES[piece as usize];

    turn = turn.enemy();
    gain[0] = attacked_piece_val;

    let lvp_sq = if let Some(sq) = least_valuable_piece(pos, attacks, turn) {
        sq
    } else {
        return 0;
    };

    let lvp_piece = pos.piece_at(&lvp_sq).unwrap().0;
    occupied ^= lvp_sq.bitboard();
    if matches!(lvp_piece, Piece::Pawn | Piece::Bishop | Piece::Queen) {
        attacks |= MOVE_GEN.bishop_moves(target_sq, occupied) & bishop_sliders;
    }

    if matches!(lvp_piece, Piece::Rook | Piece::Queen) {
        attacks |= MOVE_GEN.rook_moves(target_sq, occupied) & rook_sliders;
    }

    attacked_piece_val = PIECE_VALUES[lvp_piece as usize];
    turn = turn.enemy();

    let mut counter = 0;
    let mut remaining_attackers = attacks & occupied;
    while !remaining_attackers.is_empty() {
        let least_valuable_piece_sq =
            if let Some(sq) = least_valuable_piece(pos, remaining_attackers, turn) {
                sq
            } else {
                break;
            };
        let piece = pos.piece_at(&least_valuable_piece_sq).unwrap().0;

        occupied ^= least_valuable_piece_sq.bitboard();

        if matches!(piece, Piece::Pawn | Piece::Bishop | Piece::Queen) {
            remaining_attackers |= MOVE_GEN.bishop_moves(target_sq, occupied) & bishop_sliders;
        }

        if matches!(piece, Piece::Rook | Piece::Queen) {
            remaining_attackers |= MOVE_GEN.rook_moves(target_sq, occupied) & rook_sliders;
        }

        counter += 1;
        if counter == 32 {
            println!("{pos}");
            println!("{mv}");
            println!("{}", pos.to_fen());
        }
        gain[counter] = attacked_piece_val - gain[counter - 1];
        attacked_piece_val = PIECE_VALUES[piece as usize];
        if gain[counter] > attacked_piece_val {
            break;
        }
        turn = turn.enemy();
        remaining_attackers = attacks & occupied;
    }

    while counter > 0 {
        gain[counter - 1] = -i32::max(-gain[counter - 1], gain[counter]);
        counter -= 1;
    }

    gain[0]
}

fn least_valuable_piece(pos: &Position, pieces: Bitboard, turn: Color) -> Option<Square> {
    Piece::all()
        .into_iter()
        .map(|piece| pos.pieces[turn as usize][piece as usize] & pieces)
        .find(|bb| !bb.is_empty())
        .map(|bb| bb.lsb())
}
