mod bishop;
mod king;
mod knight;
mod magic;
mod pawn;
mod rook;

pub use bishop::generate_bishop_magic_data;
pub use king::generate_king_attacks;
pub use knight::generate_knight_attacks;
pub use pawn::generate_pawn_attacks;
pub use rook::generate_rook_magic_data;
