use colored::Colorize;

use crate::{
    attack::Attack,
    board::Board,
    castling::{CastlingRights, CastlingSide},
    color::Color,
    outcome::Outcome,
    piece::Piece,
    ply::{Ply, PlyKind},
    position::Position,
    square::Square,
};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub plies: Vec<Ply>,
    pub color_to_move: Color,
    castling_rights: CastlingRights,
    pub outcome: Option<Outcome>,
    captured_by_white: Vec<Piece>,
    captured_by_black: Vec<Piece>,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            board: Board::initial(),
            plies: vec![],
            color_to_move: Color::White,
            castling_rights: CastlingRights::all(),
            outcome: None,
            captured_by_white: vec![],
            captured_by_black: vec![],
        };
    }

    fn empty() -> Self {
        return Self {
            board: Board::empty(),
            plies: vec![],
            color_to_move: Color::White,
            castling_rights: CastlingRights::all(),
            outcome: None,
            captured_by_white: vec![],
            captured_by_black: vec![],
        };
    }

    pub fn print_captured_material(&self) {
        print!("Material captured by white: ");

        if self.captured_by_white.len() == 0 {
            println!("-");
        } else {
            println!(
                "{}",
                self.captured_by_white
                    .iter()
                    .map(|p| p.to_character(Color::Black))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        print!("Material captured by black: ");

        if self.captured_by_black.len() == 0 {
            println!("-");
        } else {
            println!(
                "{}",
                self.captured_by_black
                    .iter()
                    .map(|p| p.to_character(Color::White))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    pub fn print_in_check(&self) {
        let white_in_check = self.is_in_check(Color::White);
        let black_in_check = self.is_in_check(Color::Black);

        if white_in_check {
            println!("White is in check");
        }

        if black_in_check {
            println!("Black is in check");
        }
    }

    pub fn print_color_to_move(&self) {
        println!("{} to move", self.color_to_move.to_string());
    }

    pub fn print_board(&self) {
        let white_in_check = self.is_in_check(Color::White);
        let black_in_check = self.is_in_check(Color::Black);

        let white_king = self.find_king(Color::White);
        let black_king = self.find_king(Color::Black);

        for row in 0..=7 {
            for col in 0..=7 {
                let pos = Position::new(row, col);

                let light_square = (row + col) % 2 == 0;

                let square = format!(" {} ", self.board.get(pos).to_string()).black();

                let part_of_last_ply = if let Some(last_ply) = self.plies.last() {
                    last_ply.get_from() == pos || last_ply.get_to() == pos
                } else {
                    false
                };

                let is_king_in_check_square = if white_king == pos {
                    white_in_check
                } else if black_king == pos {
                    black_in_check
                } else {
                    false
                };

                let colored_square = if is_king_in_check_square {
                    square.on_truecolor(255, 80, 80)
                } else {
                    match (part_of_last_ply, light_square) {
                        (true, true) => square.on_truecolor(207, 209, 123),
                        (true, false) => square.on_truecolor(172, 162, 73),
                        (false, true) => square.on_truecolor(240, 217, 181),
                        (false, false) => square.on_truecolor(181, 136, 99),
                    }
                };

                print!("{}", colored_square)
            }
            print!(" {}\n", 8 - row);
        }
        println!(" a  b  c  d  e  f  g  h");
    }

    fn find_king(&self, color: Color) -> Position {
        for row in 0..=7 {
            for col in 0..=7 {
                let p = Position::new(row, col);

                if let Square::Occupied(c, Piece::King) = self.board.get(p) {
                    if c == color {
                        return p;
                    }
                }
            }
        }

        panic!("No king found");
    }

    fn find_attacks_from(&self, attacker_position: Position) -> Vec<Attack> {
        let mut attacks = vec![];

        let square = self.board.get(attacker_position);

        if let Square::Occupied(attacker_color, attacker_piece) = square {
            let opponent_color = attacker_color.opponent();

            match attacker_piece {
                Piece::Pawn => {
                    let (row_shift, en_passant_target_row) = match attacker_color {
                        Color::White => (-1, 2),
                        Color::Black => (1, 5),
                    };

                    let candidates = vec![
                        attacker_position.shift(row_shift, -1),
                        attacker_position.shift(row_shift, 1),
                    ];

                    for candidate in candidates {
                        if let Some(target_position) = candidate {
                            let target_square = self.board.get(target_position);

                            if let Square::Occupied(target_color, target_piece) = target_square {
                                if target_color == opponent_color {
                                    attacks.push(Attack::new(
                                        attacker_position,
                                        attacker_piece,
                                        target_position,
                                        Some(target_piece),
                                    ));
                                }
                            } else {
                                let mut en_passant_possible = false;

                                if target_position.row == en_passant_target_row {
                                    if let Some(last_ply) = self.plies.last() {
                                        if last_ply.piece == Piece::Pawn {
                                            if let PlyKind::Regular { from, to } = last_ply.kind {
                                                let from_one_row_above_target = (from.row as isize)
                                                    == (en_passant_target_row as isize) + row_shift;
                                                let to_one_row_below_target = (to.row as isize)
                                                    == (en_passant_target_row as isize) - row_shift;

                                                if from_one_row_above_target
                                                    && to_one_row_below_target
                                                {
                                                    attacks.push(Attack::new(
                                                        attacker_position,
                                                        attacker_piece,
                                                        target_position,
                                                        Some(Piece::Pawn),
                                                    ));

                                                    en_passant_possible = true;
                                                }
                                            }
                                        }
                                    }
                                }

                                if !en_passant_possible {
                                    attacks.push(Attack::new(
                                        attacker_position,
                                        attacker_piece,
                                        target_position,
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                }
                Piece::Rook => {
                    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

                    for direction in directions {
                        let mut target_position = attacker_position;

                        loop {
                            let shifted_position = target_position.shift(direction.0, direction.1);
                            if shifted_position.is_none() {
                                break;
                            }

                            target_position = shifted_position.unwrap();

                            let target_square = self.board.get(target_position);

                            if let Square::Occupied(target_color, target_piece) = target_square {
                                if target_color == opponent_color {
                                    attacks.push(Attack::new(
                                        attacker_position,
                                        attacker_piece,
                                        target_position,
                                        Some(target_piece),
                                    ));
                                }

                                break;
                            } else {
                                attacks.push(Attack::new(
                                    attacker_position,
                                    attacker_piece,
                                    target_position,
                                    None,
                                ));
                            }
                        }
                    }
                }
                Piece::Knight => {
                    let deltas = vec![
                        (2, 1),
                        (2, -1),
                        (-2, 1),
                        (-2, -1),
                        (1, 2),
                        (1, -2),
                        (-1, 2),
                        (-1, -2),
                    ];

                    for delta in deltas {
                        let target_position = attacker_position.shift(delta.0, delta.1);

                        if target_position.is_none() {
                            continue;
                        }

                        let target_position = target_position.unwrap();

                        let target_square = self.board.get(target_position);

                        if let Square::Occupied(target_color, target_piece) = target_square {
                            if target_color == opponent_color {
                                attacks.push(Attack::new(
                                    attacker_position,
                                    attacker_piece,
                                    target_position,
                                    Some(target_piece),
                                ));
                            }
                        } else {
                            attacks.push(Attack::new(
                                attacker_position,
                                attacker_piece,
                                target_position,
                                None,
                            ));
                        }
                    }
                }
                Piece::Bishop => {
                    let directions = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

                    for direction in directions {
                        let mut target_position = attacker_position;

                        loop {
                            let shifted_position = target_position.shift(direction.0, direction.1);
                            if shifted_position.is_none() {
                                break;
                            }

                            target_position = shifted_position.unwrap();

                            let target_square = self.board.get(target_position);

                            if let Square::Occupied(target_color, target_piece) = target_square {
                                if target_color == opponent_color {
                                    attacks.push(Attack::new(
                                        attacker_position,
                                        attacker_piece,
                                        target_position,
                                        Some(target_piece),
                                    ));
                                }

                                break;
                            } else {
                                attacks.push(Attack::new(
                                    attacker_position,
                                    attacker_piece,
                                    target_position,
                                    None,
                                ));
                            }
                        }
                    }
                }
                Piece::Queen => {
                    let directions = vec![
                        (1, 0),
                        (-1, 0),
                        (0, 1),
                        (0, -1),
                        (1, 1),
                        (-1, 1),
                        (1, -1),
                        (-1, -1),
                    ];

                    for direction in directions {
                        let mut target_position = attacker_position;

                        loop {
                            let shifted_position = target_position.shift(direction.0, direction.1);
                            if shifted_position.is_none() {
                                break;
                            }

                            target_position = shifted_position.unwrap();

                            let target_square = self.board.get(target_position);

                            if let Square::Occupied(target_color, target_piece) = target_square {
                                if target_color == opponent_color {
                                    attacks.push(Attack::new(
                                        attacker_position,
                                        attacker_piece,
                                        target_position,
                                        Some(target_piece),
                                    ));
                                }

                                break;
                            } else {
                                attacks.push(Attack::new(
                                    attacker_position,
                                    attacker_piece,
                                    target_position,
                                    None,
                                ));
                            }
                        }
                    }
                }
                Piece::King => {
                    let deltas = vec![
                        (1, 0),
                        (-1, 0),
                        (0, 1),
                        (0, -1),
                        (1, 1),
                        (-1, 1),
                        (1, -1),
                        (-1, -1),
                    ];

                    for delta in deltas {
                        let target_position = attacker_position.shift(delta.0, delta.1);

                        if target_position.is_none() {
                            continue;
                        }

                        let target_position = target_position.unwrap();

                        let target_square = self.board.get(target_position);

                        if let Square::Occupied(target_color, target_piece) = target_square {
                            if target_color == opponent_color {
                                attacks.push(Attack::new(
                                    attacker_position,
                                    attacker_piece,
                                    target_position,
                                    Some(target_piece),
                                ));
                            }
                        } else {
                            attacks.push(Attack::new(
                                attacker_position,
                                attacker_piece,
                                target_position,
                                None,
                            ));
                        }
                    }
                }
            }
        }

        attacks
    }

    fn find_attacks(&self, attacker_color: Color) -> Vec<Attack> {
        let mut attacks = vec![];

        for row in 0..=7 {
            for col in 0..=7 {
                let p = Position::new(row, col);

                let square = self.board.get(p);

                if let Square::Occupied(c, _) = square {
                    if c == attacker_color {
                        attacks.append(&mut self.find_attacks_from(p));
                    }
                }
            }
        }

        attacks
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let king_position = self.find_king(color);

        let attacks = self.find_attacks(color.opponent());

        for attack in attacks {
            if attack.target_position == king_position {
                return true;
            }
        }

        false
    }

    pub fn make_trusted_ply(&mut self, ply: Ply) {
        let Ply { color, piece, kind } = ply;

        let (home_row, opponent_home_row) = match color {
            Color::White => (7, 0),
            Color::Black => (0, 7),
        };

        match kind {
            PlyKind::Regular { from, to } => {
                self.board.set_empty(from);
                self.board.set_occupied(to, color, piece);

                if piece == Piece::Rook && from.row == home_row {
                    if from.col == 0 {
                        self.castling_rights
                            .set(color, CastlingSide::Queenside, false);
                    } else if from.col == 7 {
                        self.castling_rights
                            .set(color, CastlingSide::Kingside, false);
                    }
                } else if piece == Piece::King {
                    self.castling_rights
                        .set(color, CastlingSide::Kingside, false);
                    self.castling_rights
                        .set(color, CastlingSide::Queenside, false);
                }
            }
            PlyKind::Capture { from, to, captured } => {
                self.board.set_empty(from);
                self.board.set_occupied(to, color, piece);

                match color {
                    Color::White => self.captured_by_white.push(captured),
                    Color::Black => self.captured_by_black.push(captured),
                }

                if piece == Piece::Rook && from.row == home_row {
                    if from.col == 0 {
                        self.castling_rights
                            .set(color, CastlingSide::Queenside, false);
                    } else if from.col == 7 {
                        self.castling_rights
                            .set(color, CastlingSide::Kingside, false);
                    }
                } else if piece == Piece::King {
                    self.castling_rights
                        .set(color, CastlingSide::Kingside, false);
                    self.castling_rights
                        .set(color, CastlingSide::Queenside, false);
                }

                if captured == Piece::Rook && to.row == opponent_home_row {
                    if to.col == 0 {
                        self.castling_rights
                            .set(color.opponent(), CastlingSide::Queenside, false);
                    } else if to.col == 7 {
                        self.castling_rights
                            .set(color.opponent(), CastlingSide::Kingside, false);
                    }
                }
            }
            PlyKind::CastleKingside => {
                self.board.set_empty(Position::new(home_row, 4));
                self.board.set_empty(Position::new(home_row, 7));

                self.board
                    .set_occupied(Position::new(home_row, 6), color, Piece::King);
                self.board
                    .set_occupied(Position::new(home_row, 5), color, Piece::Rook);

                self.castling_rights
                    .set(color, CastlingSide::Kingside, false);
                self.castling_rights
                    .set(color, CastlingSide::Queenside, false);
            }
            PlyKind::CastleQueenside => {
                self.board.set_empty(Position::new(home_row, 4));
                self.board.set_empty(Position::new(home_row, 0));

                self.board
                    .set_occupied(Position::new(home_row, 2), color, Piece::King);
                self.board
                    .set_occupied(Position::new(home_row, 3), color, Piece::Rook);

                self.castling_rights
                    .set(color, CastlingSide::Queenside, false);
                self.castling_rights
                    .set(color, CastlingSide::Kingside, false);
            }
            PlyKind::EnPassant { from, to } => {
                let row_shift = match color {
                    Color::White => -1,
                    Color::Black => 1,
                };

                self.board.set_empty(from);
                self.board.set_empty(to.shift(-row_shift, 0).unwrap());

                self.board.set_occupied(to, color, piece);

                match color {
                    Color::White => self.captured_by_white.push(Piece::Pawn),
                    Color::Black => self.captured_by_black.push(Piece::Pawn),
                }
            }
            PlyKind::Promotion {
                from,
                to,
                promotion,
            } => {
                self.board.set_empty(from);
                self.board.set_occupied(to, color, promotion);
            }
            PlyKind::PromotionCapture {
                from,
                to,
                promotion,
                captured,
            } => {
                self.board.set_empty(from);
                self.board.set_occupied(to, color, promotion);

                match color {
                    Color::White => self.captured_by_white.push(captured),
                    Color::Black => self.captured_by_black.push(captured),
                }

                if captured == Piece::Rook && to.row == opponent_home_row {
                    if to.col == 0 {
                        self.castling_rights
                            .set(color.opponent(), CastlingSide::Queenside, false);
                    } else if to.col == 7 {
                        self.castling_rights
                            .set(color.opponent(), CastlingSide::Kingside, false);
                    }
                }
            }
        }

        self.plies.push(ply);
        self.color_to_move = self.color_to_move.opponent();
    }

    fn find_legal_plies_from(&self, position: Position) -> Vec<Ply> {
        let square = self.board.get(position);

        match square {
            Square::Empty => vec![],
            Square::Occupied(color, piece) => {
                let mut plies = vec![];

                let opponent_color = color.opponent();

                match piece {
                    Piece::Pawn => {
                        let (
                            row_shift,
                            pawn_row,
                            en_passant_target_from_row,
                            en_passant_target_to_row,
                        ) = match color {
                            Color::White => (-1, 6, 1, 3),
                            Color::Black => (1, 1, 6, 4),
                        };

                        let can_promote = match color {
                            Color::White => position.row == 1,
                            Color::Black => position.row == 6,
                        };

                        if can_promote {
                            let promotion_candidates =
                                vec![Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen];

                            let target_position = position.shift(row_shift, 0).unwrap();
                            let target_square = self.board.get(target_position);

                            if let Square::Empty = target_square {
                                for promotion_candidate in &promotion_candidates {
                                    plies.push(Ply::new(
                                        color,
                                        piece,
                                        PlyKind::Promotion {
                                            from: position,
                                            to: target_position,
                                            promotion: *promotion_candidate,
                                        },
                                    ));
                                }
                            }

                            let capture_candidates =
                                vec![position.shift(row_shift, -1), position.shift(row_shift, 1)];

                            for capture_candidate in capture_candidates {
                                if let Some(target_position) = capture_candidate {
                                    let target_square = self.board.get(target_position);

                                    if let Square::Occupied(target_color, target_piece) =
                                        target_square
                                    {
                                        if target_color == opponent_color
                                            && target_piece != Piece::King
                                        {
                                            for promotion_candidate in &promotion_candidates {
                                                plies.push(Ply::new(
                                                    color,
                                                    piece,
                                                    PlyKind::PromotionCapture {
                                                        from: position,
                                                        to: target_position,
                                                        promotion: *promotion_candidate,
                                                        captured: target_piece,
                                                    },
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(target_position) = position.shift(row_shift, 0) {
                                let target_square = self.board.get(target_position);

                                if let Square::Empty = target_square {
                                    plies.push(Ply::new(
                                        color,
                                        piece,
                                        PlyKind::Regular {
                                            from: position,
                                            to: target_position,
                                        },
                                    ));

                                    if position.row == pawn_row {
                                        if let Some(second_target_position) =
                                            target_position.shift(row_shift, 0)
                                        {
                                            let second_target_square =
                                                self.board.get(second_target_position);

                                            if let Square::Empty = second_target_square {
                                                plies.push(Ply::new(
                                                    color,
                                                    piece,
                                                    PlyKind::Regular {
                                                        from: position,
                                                        to: second_target_position,
                                                    },
                                                ));
                                            }
                                        }
                                    }
                                }
                            }

                            let capture_candidates =
                                vec![position.shift(row_shift, -1), position.shift(row_shift, 1)];

                            for capture_candidate in capture_candidates {
                                if let Some(target_position) = capture_candidate {
                                    let target_square = self.board.get(target_position);

                                    if let Square::Occupied(target_color, target_piece) =
                                        target_square
                                    {
                                        if target_color == opponent_color
                                            && target_piece != Piece::King
                                        {
                                            plies.push(Ply::new(
                                                color,
                                                piece,
                                                PlyKind::Capture {
                                                    from: position,
                                                    to: target_position,
                                                    captured: target_piece,
                                                },
                                            ));
                                        }
                                    }
                                }
                            }

                            if position.row == en_passant_target_to_row {
                                if let Some(last_ply) = self.plies.last() {
                                    if last_ply.piece == Piece::Pawn {
                                        if let PlyKind::Regular { from, to } = last_ply.kind {
                                            let moved_two_rows = from.row
                                                == en_passant_target_from_row
                                                && to.row == en_passant_target_to_row;
                                            let is_next_to_attacker =
                                                (to.col as isize - position.col as isize).abs()
                                                    == 1;

                                            if moved_two_rows && is_next_to_attacker {
                                                plies.push(Ply::new(
                                                    color,
                                                    piece,
                                                    PlyKind::EnPassant {
                                                        from: position,
                                                        to: to.shift(row_shift, 0).unwrap(),
                                                    },
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Piece::Rook => {
                        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

                        for direction in directions {
                            let mut target_position = position;

                            loop {
                                let shifted_position =
                                    target_position.shift(direction.0, direction.1);
                                if shifted_position.is_none() {
                                    break;
                                }

                                target_position = shifted_position.unwrap();
                                let target_square = self.board.get(target_position);

                                match target_square {
                                    Square::Empty => {
                                        plies.push(Ply::new(
                                            color,
                                            piece,
                                            PlyKind::Regular {
                                                from: position,
                                                to: target_position,
                                            },
                                        ));
                                    }
                                    Square::Occupied(target_color, target_piece) => {
                                        if target_color == opponent_color
                                            && target_piece != Piece::King
                                        {
                                            plies.push(Ply::new(
                                                color,
                                                piece,
                                                PlyKind::Capture {
                                                    from: position,
                                                    to: target_position,
                                                    captured: target_piece,
                                                },
                                            ));
                                        }

                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Piece::Knight => {
                        let deltas = vec![
                            (2, 1),
                            (2, -1),
                            (-2, 1),
                            (-2, -1),
                            (1, 2),
                            (1, -2),
                            (-1, 2),
                            (-1, -2),
                        ];

                        for delta in deltas {
                            let shifted_position = position.shift(delta.0, delta.1);

                            if shifted_position.is_none() {
                                continue;
                            }

                            let target_position = shifted_position.unwrap();
                            let target_square = self.board.get(target_position);

                            match target_square {
                                Square::Empty => {
                                    plies.push(Ply::new(
                                        color,
                                        piece,
                                        PlyKind::Regular {
                                            from: position,
                                            to: target_position,
                                        },
                                    ));
                                }
                                Square::Occupied(target_color, target_piece) => {
                                    if target_color == opponent_color && target_piece != Piece::King
                                    {
                                        plies.push(Ply::new(
                                            color,
                                            piece,
                                            PlyKind::Capture {
                                                from: position,
                                                to: target_position,
                                                captured: target_piece,
                                            },
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    Piece::Bishop => {
                        let directions = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

                        for direction in directions {
                            let mut target_position = position;

                            loop {
                                let shifted_position =
                                    target_position.shift(direction.0, direction.1);
                                if shifted_position.is_none() {
                                    break;
                                }

                                target_position = shifted_position.unwrap();
                                let target_square = self.board.get(target_position);

                                match target_square {
                                    Square::Empty => {
                                        plies.push(Ply::new(
                                            color,
                                            piece,
                                            PlyKind::Regular {
                                                from: position,
                                                to: target_position,
                                            },
                                        ));
                                    }
                                    Square::Occupied(target_color, target_piece) => {
                                        if target_color == opponent_color
                                            && target_piece != Piece::King
                                        {
                                            plies.push(Ply::new(
                                                color,
                                                piece,
                                                PlyKind::Capture {
                                                    from: position,
                                                    to: target_position,
                                                    captured: target_piece,
                                                },
                                            ));
                                        }

                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Piece::Queen => {
                        let directions = vec![
                            (1, 0),
                            (-1, 0),
                            (0, 1),
                            (0, -1),
                            (1, 1),
                            (-1, 1),
                            (1, -1),
                            (-1, -1),
                        ];

                        for direction in directions {
                            let mut target_position = position;

                            loop {
                                let shifted_position =
                                    target_position.shift(direction.0, direction.1);
                                if shifted_position.is_none() {
                                    break;
                                }

                                target_position = shifted_position.unwrap();
                                let target_square = self.board.get(target_position);

                                match target_square {
                                    Square::Empty => {
                                        plies.push(Ply::new(
                                            color,
                                            piece,
                                            PlyKind::Regular {
                                                from: position,
                                                to: target_position,
                                            },
                                        ));
                                    }
                                    Square::Occupied(target_color, target_piece) => {
                                        if target_color == opponent_color
                                            && target_piece != Piece::King
                                        {
                                            plies.push(Ply::new(
                                                color,
                                                piece,
                                                PlyKind::Capture {
                                                    from: position,
                                                    to: target_position,
                                                    captured: target_piece,
                                                },
                                            ));
                                        }

                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Piece::King => {
                        let deltas = vec![
                            (1, 0),
                            (-1, 0),
                            (0, 1),
                            (0, -1),
                            (1, 1),
                            (-1, 1),
                            (1, -1),
                            (-1, -1),
                        ];

                        for delta in deltas {
                            let shifted_position = position.shift(delta.0, delta.1);

                            if shifted_position.is_none() {
                                continue;
                            }

                            let target_position = shifted_position.unwrap();
                            let target_square = self.board.get(target_position);

                            match target_square {
                                Square::Empty => {
                                    plies.push(Ply::new(
                                        color,
                                        piece,
                                        PlyKind::Regular {
                                            from: position,
                                            to: target_position,
                                        },
                                    ));
                                }
                                Square::Occupied(target_color, target_piece) => {
                                    if target_color == opponent_color && target_piece != Piece::King
                                    {
                                        plies.push(Ply::new(
                                            color,
                                            piece,
                                            PlyKind::Capture {
                                                from: position,
                                                to: target_position,
                                                captured: target_piece,
                                            },
                                        ));
                                    }
                                }
                            }
                        }

                        let (castle_kingside_allowed, castle_queenside_allowed) = (
                            self.castling_rights.get(color, CastlingSide::Kingside),
                            self.castling_rights.get(color, CastlingSide::Queenside),
                        );

                        if castle_kingside_allowed || castle_queenside_allowed {
                            let attacks = self.find_attacks(color.opponent());

                            let home_row = match color {
                                Color::White => 7,
                                Color::Black => 0,
                            };

                            if castle_kingside_allowed {
                                let need_to_be_empty =
                                    vec![Position::new(home_row, 5), Position::new(home_row, 6)];

                                let need_to_be_unattacked = vec![
                                    Position::new(home_row, 4),
                                    Position::new(home_row, 5),
                                    Position::new(home_row, 6),
                                ];

                                let can_castle_kingside = need_to_be_empty
                                    .iter()
                                    .all(|p| self.board.get(*p) == Square::Empty)
                                    && attacks.iter().all(|a| {
                                        !need_to_be_unattacked.contains(&a.target_position)
                                    });

                                if can_castle_kingside {
                                    plies.push(Ply::new(color, piece, PlyKind::CastleKingside));
                                }
                            }

                            if castle_queenside_allowed {
                                let need_to_be_empty = vec![
                                    Position::new(home_row, 1),
                                    Position::new(home_row, 2),
                                    Position::new(home_row, 3),
                                ];

                                let need_to_be_unattacked = vec![
                                    Position::new(home_row, 2),
                                    Position::new(home_row, 3),
                                    Position::new(home_row, 4),
                                ];

                                let can_castle_queenside = need_to_be_empty
                                    .iter()
                                    .all(|p| self.board.get(*p) == Square::Empty)
                                    && attacks.iter().all(|a| {
                                        !need_to_be_unattacked.contains(&a.target_position)
                                    });

                                if can_castle_queenside {
                                    plies.push(Ply::new(color, piece, PlyKind::CastleQueenside));
                                }
                            }
                        }
                    }
                }

                let mut legal_plies = vec![];

                for ply in plies {
                    let mut game = self.clone();

                    game.make_trusted_ply(ply.clone());

                    if !game.is_in_check(color) {
                        legal_plies.push(ply);
                    }
                }

                legal_plies
            }
        }
    }

    pub fn find_legal_plies(&self, color: Color) -> Vec<Ply> {
        if self.outcome.is_some() {
            return vec![];
        }

        let mut plies = vec![];

        for row in 0..=7 {
            for col in 0..=7 {
                let p = Position::new(row, col);

                let square = self.board.get(p);

                if let Square::Occupied(c, _) = square {
                    if c == color {
                        plies.append(&mut self.find_legal_plies_from(p));
                    }
                }
            }
        }

        plies
    }

    pub fn from_fen(fen: String) -> Result<Game, String> {
        let fen = fen.trim();

        let sections = fen.split(" ").collect::<Vec<_>>();

        if sections.len() < 4 {
            return Err("Invalid FEN (less than 4 sections)".into());
        }

        let rows = sections[0].split("/").collect::<Vec<_>>();
        let color_to_move = sections[1];
        let castling_rights = sections[2];
        let en_passant_target = sections[3];

        let mut game = Game::empty();

        if rows.len() != 8 {
            return Err("Invalid FEN (wrong number of ranks)".into());
        }

        for (row_index, row) in rows.iter().enumerate() {
            let mut col_index = 0;

            for c in row.chars() {
                if col_index > 7 {
                    return Err("Invalid FEN (too many squares in rank)".into());
                }

                if let Some(n) = c.to_digit(10) {
                    col_index += n as usize;
                } else {
                    let color = if c.is_ascii_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    let piece = match c.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'r' => Piece::Rook,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => {
                            return Err(format!("Invalid FEN (invalid piece: {})", c).into());
                        }
                    };

                    game.board
                        .set_occupied(Position::new(row_index, col_index), color, piece);

                    col_index += 1;
                }
            }

            if col_index != 8 {
                return Err("Invalid FEN (too few squares in rank)".into());
            }
        }

        let color_to_move = match color_to_move {
            "w" => Color::White,
            "b" => Color::Black,
            _ => {
                return Err(
                    format!("Invalid FEN (invalid color to move: {})", color_to_move).into(),
                );
            }
        };

        game.color_to_move = color_to_move;

        game.castling_rights = CastlingRights::none();
        if castling_rights != "-" {
            for c in castling_rights.chars() {
                let color = if c.is_ascii_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };

                let side = match c.to_ascii_lowercase() {
                    'k' => CastlingSide::Kingside,
                    'q' => CastlingSide::Queenside,
                    _ => {
                        return Err(format!("Invalid FEN (invalid castling rights: {})", c).into());
                    }
                };

                game.castling_rights.set(color, side, true);
            }
        }

        if en_passant_target != "-" {
            let en_passant_target_position = Position::from_chess(en_passant_target);

            if en_passant_target_position.is_none() {
                return Err(format!(
                    "Invalid FEN (invalid en passant target: {})",
                    en_passant_target
                )
                .into());
            }

            let pos = en_passant_target_position.unwrap();

            let (en_passant_from_row, en_passant_through_row, en_passant_to_row) =
                match color_to_move {
                    Color::White => (1, 2, 3),
                    Color::Black => (6, 5, 4),
                };

            let from_square_is_empty =
                game.board.get(Position::new(en_passant_from_row, pos.col)) == Square::Empty;
            let through_square_is_empty = game
                .board
                .get(Position::new(en_passant_through_row, pos.col))
                == Square::Empty;
            let to_square_is_opponent_pawn =
                game.board.get(Position::new(en_passant_to_row, pos.col))
                    == Square::Occupied(color_to_move.opponent(), Piece::Pawn);

            if !(pos.row == en_passant_through_row
                && from_square_is_empty
                && through_square_is_empty
                && to_square_is_opponent_pawn)
            {
                return Err(format!(
                    "Invalid FEN (invalid en passant target: {})",
                    en_passant_target
                )
                .into());
            }

            game.plies.push(Ply::new(
                color_to_move.opponent(),
                Piece::Pawn,
                PlyKind::Regular {
                    from: Position::new(en_passant_from_row, pos.col),
                    to: Position::new(en_passant_to_row, pos.col),
                },
            ));
        }

        Ok(game)
    }
}
