use crate::{color::Color, piece::Piece, position::Position};

#[derive(Debug, Clone, Copy)]
pub struct Ply {
    pub color: Color,
    pub piece: Piece,
    pub kind: PlyKind,
}

#[derive(Debug, Clone, Copy)]
pub enum PlyKind {
    Regular {
        from: Position,
        to: Position,
    },
    Capture {
        from: Position,
        to: Position,
        captured: Piece,
    },
    CastleKingside,
    CastleQueenside,
    EnPassant {
        from: Position,
        to: Position,
    },
    Promotion {
        from: Position,
        to: Position,
        promotion: Piece,
    },
    PromotionCapture {
        from: Position,
        to: Position,
        promotion: Piece,
        captured: Piece,
    },
}

impl Ply {
    pub fn new(color: Color, piece: Piece, kind: PlyKind) -> Self {
        return Self { color, piece, kind };
    }

    pub fn get_from(&self) -> Position {
        match self.kind {
            PlyKind::Regular { from, .. } => from,
            PlyKind::Capture { from, .. } => from,
            PlyKind::CastleKingside | PlyKind::CastleQueenside => match self.color {
                Color::White => Position::new(7, 4),
                Color::Black => Position::new(0, 4),
            },
            PlyKind::EnPassant { from, .. } => from,
            PlyKind::Promotion { from, .. } => from,
            PlyKind::PromotionCapture { from, .. } => from,
        }
    }

    pub fn get_to(&self) -> Position {
        match self.kind {
            PlyKind::Regular { to, .. } => to,
            PlyKind::Capture { to, .. } => to,
            PlyKind::CastleKingside => match self.color {
                Color::White => Position::new(7, 6),
                Color::Black => Position::new(0, 6),
            },
            PlyKind::CastleQueenside => match self.color {
                Color::White => Position::new(7, 2),
                Color::Black => Position::new(0, 2),
            },
            PlyKind::EnPassant { to, .. } => to,
            PlyKind::Promotion { to, .. } => to,
            PlyKind::PromotionCapture { to, .. } => to,
        }
    }

    pub fn to_string(&self) -> String {
        match self.kind {
            PlyKind::Regular { from, to } => format!(
                "{} ({}) to {}",
                from.to_chess().unwrap(),
                self.piece.to_name(),
                to.to_chess().unwrap()
            ),
            PlyKind::Capture { from, to, captured } => format!(
                "{} ({}) captures {} ({})",
                from.to_chess().unwrap(),
                self.piece.to_name(),
                to.to_chess().unwrap(),
                captured.to_name(),
            ),
            PlyKind::CastleKingside => "Castle Kingside".to_string(),
            PlyKind::CastleQueenside => "Castle Queenside".to_string(),
            PlyKind::EnPassant { from, to } => format!(
                "{} ({}) captures {} (En Passant)",
                from.to_chess().unwrap(),
                self.piece.to_name(),
                to.to_chess().unwrap()
            ),
            PlyKind::Promotion {
                from,
                to,
                promotion,
            } => format!(
                "{} ({}) to {} (Promotion to {})",
                from.to_chess().unwrap(),
                self.piece.to_name(),
                to.to_chess().unwrap(),
                promotion.to_name()
            ),
            PlyKind::PromotionCapture {
                from,
                to,
                promotion,
                captured,
            } => format!(
                "{} ({}) captures {} ({}) (Promotion to {})",
                from.to_chess().unwrap(),
                self.piece.to_name(),
                to.to_chess().unwrap(),
                captured.to_name(),
                promotion.to_name()
            ),
        }
    }
}
