mod bishop;
mod king;
mod knight;
mod occupancy;
mod pawn;
mod queen;
mod rook;

use std::fmt::Display;

pub use bishop::*;
pub use king::*;
pub use knight::*;
pub use occupancy::*;
pub use pawn::*;
pub use queen::*;
pub use rook::*;

use crate::{
    bitboard::{rf_to_square, square_to_string},
    enums::{Color, PieceAndColor},
    position::Position,
};

#[derive(Clone)]
pub struct Move {
    pub source_square: u8,
    pub target_square: u8,
    pub piece: PieceAndColor,
    pub promotion_piece: Option<PieceAndColor>,
    pub is_capture: bool,
    pub is_en_passant: bool,
    pub is_castling: bool,
    pub is_double_pawn_push: bool,
}

impl Move {
    pub fn new(
        source_square: u8,
        target_square: u8,
        piece: PieceAndColor,
        promotion_piece: Option<PieceAndColor>,
        is_capture: bool,
        is_en_passant: bool,
        is_castling: bool,
        is_double_pawn_push: bool,
    ) -> Self {
        Self {
            source_square,
            target_square,
            piece,
            promotion_piece,
            is_capture,
            is_en_passant,
            is_castling,
            is_double_pawn_push,
        }
    }

    pub fn to_uci(&self) -> String {
        let mut s = square_to_string(self.source_square).unwrap();
        s.push_str(&square_to_string(self.target_square).unwrap());

        if let Some(promotion_piece) = self.promotion_piece {
            s.push_str(&promotion_piece.to_char().to_string());
        }

        s
    }

    pub fn to_pretty_print(&self) -> String {
        let mut s = square_to_string(self.source_square).unwrap();
        s.push_str(&square_to_string(self.target_square).unwrap());

        if let Some(promotion_piece) = self.promotion_piece {
            s.push_str(&format!(" [PROMOTION: {}]", promotion_piece));
        }

        if self.is_capture {
            s.push_str(" [CAPTURE]");
        }

        if self.is_en_passant {
            s.push_str(" [EN PASSANT]");
        }

        if self.is_castling {
            s.push_str(" [CASTLING]");
        }

        if self.is_double_pawn_push {
            s.push_str(" [DOUBLE PAWN PUSH]");
        }

        s
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

pub struct MoveGen {
    pawn: PawnMoveGen,
    knight: KnightMoveGen,
    king: KingMoveGen,
    rook: RookMoveGen,
    bishop: BishopMoveGen,
    queen: QueenMoveGen,
}

impl MoveGen {
    pub fn pawn(&self) -> &PawnMoveGen {
        &self.pawn
    }

    pub fn knight(&self) -> &KnightMoveGen {
        &self.knight
    }

    pub fn king(&self) -> &KingMoveGen {
        &self.king
    }

    pub fn rook(&self) -> &RookMoveGen {
        &self.rook
    }

    pub fn bishop(&self) -> &BishopMoveGen {
        &self.bishop
    }

    pub fn queen(&self) -> &QueenMoveGen {
        &self.queen
    }

    pub fn new() -> Self {
        let rook = RookMoveGen::new();
        let bishop = BishopMoveGen::new();

        Self {
            pawn: PawnMoveGen::new(),
            knight: KnightMoveGen::new(),
            king: KingMoveGen::new(),
            rook: rook.clone(),
            bishop: bishop.clone(),
            queen: QueenMoveGen::new(rook, bishop),
        }
    }

    pub fn is_square_attacked(
        &self,
        position: &Position,
        square: u8,
        attacker_color: Color,
    ) -> bool {
        if attacker_color == Color::Both {
            unreachable!();
        }

        if attacker_color == Color::White {
            if self.pawn.attack_table()[Color::Black as usize][square as usize]
                & position.bitboards().piece(PieceAndColor::WhitePawn)
                != 0
            {
                return true;
            }

            if self.knight.attack_table()[square as usize]
                & position.bitboards().piece(PieceAndColor::WhiteKnight)
                != 0
            {
                return true;
            }

            if self.king.attack_table()[square as usize]
                & position.bitboards().piece(PieceAndColor::WhiteKing)
                != 0
            {
                return true;
            }

            if self
                .rook
                .get_attacks(square, position.bitboards().color(Color::Both))
                & position.bitboards().piece(PieceAndColor::WhiteRook)
                != 0
            {
                return true;
            }

            if self
                .bishop
                .get_attacks(square, position.bitboards().color(Color::Both))
                & position.bitboards().piece(PieceAndColor::WhiteBishop)
                != 0
            {
                return true;
            }

            if self
                .queen
                .get_attacks(square, position.bitboards().color(Color::Both))
                & position.bitboards().piece(PieceAndColor::WhiteQueen)
                != 0
            {
                return true;
            }
        } else {
            if self.pawn.attack_table()[Color::White as usize][square as usize]
                & position.bitboards().piece(PieceAndColor::BlackPawn)
                != 0
            {
                return true;
            }

            if self.knight.attack_table()[square as usize]
                & position.bitboards().piece(PieceAndColor::BlackKnight)
                != 0
            {
                return true;
            }

            if self.king.attack_table()[square as usize]
                & position.bitboards().piece(PieceAndColor::BlackKing)
                != 0
            {
                return true;
            }

            if self
                .rook
                .get_attacks(square, position.bitboards().color(Color::Both))
                & position.bitboards().piece(PieceAndColor::BlackRook)
                != 0
            {
                return true;
            }

            if self
                .bishop
                .get_attacks(square, position.bitboards().color(Color::Both))
                & position.bitboards().piece(PieceAndColor::BlackBishop)
                != 0
            {
                return true;
            }

            if self
                .queen
                .get_attacks(square, position.bitboards().color(Color::Both))
                & position.bitboards().piece(PieceAndColor::BlackQueen)
                != 0
            {
                return true;
            }
        }

        false
    }

    pub fn print_attacked_squares(&self, position: &Position, attacker_color: Color) {
        for rank in 0..8 {
            print!("{} ", 8 - rank);

            for file in 0..8 {
                let square = rf_to_square(rank, file);

                print!(
                    " {}",
                    if self.is_square_attacked(position, square, attacker_color) {
                        "1"
                    } else {
                        "0"
                    }
                );
            }

            println!();
        }

        println!("\n   a b c d e f g h");
    }

    pub fn generate_pseudo_legal_moves(&self, position: &Position) -> Vec<Move> {
        if position.color_to_move() == Color::Both {
            unreachable!();
        }

        let mut moves = Vec::new();

        moves.append(&mut self.pawn.generate_moves(position));
        moves.append(&mut self.knight.generate_moves(position));
        moves.append(&mut self.king.generate_moves(self, position));
        moves.append(&mut self.rook.generate_moves(position));
        moves.append(&mut self.bishop.generate_moves(position));
        moves.append(&mut self.queen.generate_moves(position));

        moves
    }

    pub fn generate_legal_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        let pseudo_legal_moves = self.generate_pseudo_legal_moves(position);

        for pseudo_legal_move in pseudo_legal_moves {
            let opt = position.make_move(self, &pseudo_legal_move, false);

            if opt.is_some() {
                moves.push(pseudo_legal_move);
            }
        }

        moves
    }

    pub fn parse_uci_move(&self, position: &Position, move_str: String) -> Option<Move> {
        let legal_moves = self.generate_legal_moves(position);

        for legal_move in legal_moves {
            if legal_move.to_uci().to_ascii_lowercase() == move_str.to_ascii_lowercase() {
                return Some(legal_move);
            }
        }

        None
    }
}
