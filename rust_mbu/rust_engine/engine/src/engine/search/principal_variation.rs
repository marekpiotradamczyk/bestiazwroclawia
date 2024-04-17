use crate::engine::search::MAX_PLY;
use move_gen::r#move::Move;

const PV_TABLE_SIZE: usize = MAX_PLY;

#[derive(Clone)]
pub struct PrincipalVariation {
    table: Vec<Vec<Option<Move>>>,
    length: Vec<usize>,
}

impl Default for PrincipalVariation {
    fn default() -> Self {
        Self {
            table: vec![vec![None; PV_TABLE_SIZE]; PV_TABLE_SIZE],
            length: vec![0; PV_TABLE_SIZE],
        }
    }
}

impl PrincipalVariation {
    pub fn init_length(&mut self, ply: usize) {
        self.length[ply] = ply;
    }

    pub fn push_pv_move(&mut self, ply: usize, mv: Move) {
        self.table[ply][ply] = Some(mv);

        for i in ply + 1..self.length[ply + 1] {
            self.table[ply][i] = self.table[ply + 1][i];
        }

        self.length[ply] = self.length[ply + 1];
    }

    #[must_use]
    pub fn best(&self) -> Option<Move> {
        self.table[0][0]
    }

    #[must_use]
    pub fn is_only_legal_move(&self) -> bool {
        self.length[0] == 1
    }
}

impl ToString for PrincipalVariation {
    fn to_string(&self) -> String {
        let mut pv = String::new();

        for i in 0..self.length[0] {
            pv.push_str(&format!("{} ", self.table[0][i].unwrap()));
        }

        pv
    }
}
