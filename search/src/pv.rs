use psce_core::Move;

const MAX_PLY: usize = 64;

#[derive(Debug)]
pub struct PrincipalVariations {
    table: Box<[[Option<Move>; MAX_PLY]; MAX_PLY]>,
    length: [usize; MAX_PLY],
}

impl PrincipalVariations {
    pub fn new() -> Self {
        Self {
            table: Box::new([[None; MAX_PLY]; MAX_PLY]),
            length: [0; MAX_PLY],
        }
    }

    pub fn update(&mut self, ply: usize, mv: Move) {
        let child_length = self.length[ply + 1];

        let (left, right) = self.table.split_at_mut(ply + 1);

        left[ply][0] = Some(mv);
        left[ply][1..=child_length].copy_from_slice(&right[0][0..child_length]);

        self.length[ply] = child_length + 1;
    }

    pub fn clear_ply(&mut self, ply: usize) {
        self.length[ply] = 0;
    }

    pub fn get_pv(&self, ply: usize) -> Vec<Move> {
        self.table[ply][0..self.length[ply]]
            .iter()
            .map(|mv| mv.unwrap())
            .collect()
    }
}
