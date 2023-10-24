use crate::color::Color;

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Checkmate { winner: Color },
    Stalemate,
    Resignation { winner: Color },
    Draw,
}

impl Outcome {
    pub fn to_string(self) -> String {
        match self {
            Self::Checkmate { winner } => format!("Checkmate: {} wins", winner.to_string()),
            Self::Stalemate => "Stalemate".to_string(),
            Self::Resignation { winner } => format!("Resignation: {} wins", winner.to_string()),
            Self::Draw => "Draw".to_string(),
        }
    }
}
