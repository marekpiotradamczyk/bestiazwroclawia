use rand::Rng;
use sdk::{
    bitboard::{Bitboard, Direction},
    lookup::{in_between, sliders::Slider},
    position::{self, Color, Piece, Position},
    square::{Rank, Square},
};

use crate::{
    lookup::{load_lookup_tables, LookupTables, MagicEntry},
    r#move::{MakeMove, Move, MoveKind},
    xray::XRayGenerator,
};

use super::pieces::{
    king_generator::KingMoveGenerator, knight_generator::KnightMoveGenerator,
    pawn_generator::PawnMoveGenerator, simple_move_generator::SimpleMoveGenerator,
    slider_generator::SliderMoveGenerator,
};

pub struct MoveGen {
    pub lookups: LookupTables,
}

impl MoveGen {
    pub fn new() -> Self {
        let lookup_tables = load_lookup_tables().expect("Couldn't load lookup tables");
        Self {
            lookups: lookup_tables,
        }
    }

    pub fn pinned_pieces(&self, pos: &Position) -> Bitboard {
        let king_square = pos.pieces[pos.turn as usize][Piece::King as usize].msb();
        let occ = pos.occupied;

        let own_pieces = pos.occupation(&pos.turn);

        let op_rq = pos.pieces[pos.enemy() as usize][Piece::Rook as usize]
            | pos.pieces[pos.enemy() as usize][Piece::Queen as usize];

        let op_bq = pos.pieces[pos.enemy() as usize][Piece::Bishop as usize]
            | pos.pieces[pos.enemy() as usize][Piece::Queen as usize];

        let mut pinned_pieces = Bitboard(0);

        for sq in self.xray_rook_attacks(king_square, occ) & op_rq {
            pinned_pieces |=
                self.lookups.in_between[sq as usize][king_square as usize] & own_pieces;
        }

        for sq in self.xray_bishop_attacks(king_square, occ) & op_bq {
            pinned_pieces |=
                self.lookups.in_between[sq as usize][king_square as usize] & own_pieces;
        }

        pinned_pieces
    }

    pub fn attacks_to_square(
        &self,
        position: &Position,
        sq: Square,
        by_side: Color,
        occupied: Bitboard,
    ) -> Bitboard {
        let opposite_color = by_side.enemy();
        let pieces = position.pieces[by_side as usize];

        let pawns = pieces[Piece::Pawn as usize];
        let pawns_attacks = self.pawn_attacks(opposite_color, sq) & pawns;

        let knights = pieces[Piece::Knight as usize];
        let knights_attacks = self.knight_attacks(sq) & knights;

        let king = pieces[Piece::King as usize];
        let king_attacks = self.king_attacks(sq) & king;

        let bishop_queens = pieces[Piece::Bishop as usize] | pieces[Piece::Queen as usize];
        let bishop_attacks = self.bishop_moves(sq, occupied) & bishop_queens;

        let rook_queens = pieces[Piece::Rook as usize] | pieces[Piece::Queen as usize];
        let rook_attacks = self.rook_moves(sq, occupied) & rook_queens;

        pawns_attacks | knights_attacks | king_attacks | bishop_attacks | rook_attacks
    }

    pub fn is_check(&self, position: &Position) -> bool {
        !self
            .attacks_to_square(
                position,
                position.pieces[position.turn as usize][Piece::King as usize].msb(),
                position.enemy(),
                position.occupied,
            )
            .is_empty()
    }

    pub fn is_double_check(&self, position: &Position) -> bool {
        let mut attacked_bb = self.attacks_to_square(
            position,
            position.pieces[position.turn as usize][Piece::King as usize].msb(),
            position.enemy(),
            position.occupied,
        );

        if !attacked_bb.is_empty() {
            attacked_bb.pop_lsb();

            !attacked_bb.is_empty()
        } else {
            false
        }
    }

    pub fn generate_legal_moves<'a>(
        &'a self,
        pos: &'a Position,
    ) -> Box<dyn Iterator<Item = Move> + 'a> {
        let friendly_occ = pos.occupation(&pos.turn);
        let enemy_occ = pos.occupation(&pos.enemy());
        let pinned_pieces = self.pinned_pieces(pos);
        let king_square = pos.pieces[pos.turn as usize][Piece::King as usize].msb();

        let pawn_quiet_moves =
            self.generate_pawn_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let pawn_capturing_moves =
            self.generate_pawn_attacks(pos, friendly_occ, enemy_occ, pinned_pieces);
        let knight_moves = self.generate_knight_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let slider_moves = self.generate_slider_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let king_moves = self.generate_king_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let castling_moves =
            self.generate_all_castlings(pos, friendly_occ, enemy_occ, pinned_pieces);

        let mut attackers_to_king =
            self.attacks_to_square(pos, king_square, pos.enemy(), pos.occupied);

        let non_king_moves = pawn_quiet_moves
            .chain(pawn_capturing_moves)
            .chain(knight_moves)
            .chain(slider_moves)
            .chain(castling_moves);

        if attackers_to_king.is_empty() {
            Box::new(non_king_moves.chain(king_moves))
        } else {
            let attacker_sq = attackers_to_king.pop_lsb();

            if attackers_to_king.is_empty() {
                let slider = match pos.piece_at(&attacker_sq).unwrap().0 {
                    Piece::Rook => Some(Slider::Rook),
                    Piece::Bishop => Some(Slider::Bishop),
                    Piece::Queen => Some(Slider::Queen),
                    _ => None,
                };

                Box::new(non_king_moves.chain(king_moves).filter(move |mv| {
                    let blockable_squares = if slider.is_some() {
                        let between =
                            self.lookups.in_between[attacker_sq as usize][king_square as usize];

                        between & !pos.occupation(&pos.turn)
                    } else {
                        Bitboard::empty()
                    };

                    mv.to() == attacker_sq
                        || mv.from() == king_square
                        || !(mv.to().bitboard() & blockable_squares).is_empty()
                }))
            } else {
                Box::new(king_moves)
            }
        }
    }
}

pub enum PositionState {
    Checkmate,
    Stalemate,
    InProgress,
}

impl Default for MoveGen {
    fn default() -> Self {
        Self::new()
    }
}
