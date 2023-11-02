use super::search::MAX_PLY;
use move_gen::r#move::Move;

pub struct PrincipalVariation {
    table: [[Option<Move>; MAX_PLY]; MAX_PLY],
    length: [usize; MAX_PLY],
}

impl Default for PrincipalVariation {
    fn default() -> Self {
        Self {
            table: [[None; MAX_PLY]; MAX_PLY],
            length: [0; MAX_PLY],
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
