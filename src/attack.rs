use crate::{piece::Piece, position::Position};

#[derive(Debug, Clone)]
pub struct Attack {
    pub attacker_position: Position,
    pub attacker_piece: Piece,
    pub target_position: Position,
    pub target_piece: Option<Piece>,
}

impl Attack {
    pub fn new(
        attacker_position: Position,
        attacker_piece: Piece,
        target_position: Position,
        target_piece: Option<Piece>,
    ) -> Self {
        return Self {
            attacker_position,
            attacker_piece,
            target_position,
            target_piece,
        };
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{} ({}) to {} ({})",
            self.attacker_position.to_chess().unwrap(),
            self.attacker_piece.to_name(),
            self.target_position.to_chess().unwrap(),
            match self.target_piece {
                Some(p) => p.to_name(),
                None => "Empty".to_string(),
            }
        );
    }
}
