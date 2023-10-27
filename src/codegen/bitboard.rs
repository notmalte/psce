use crate::bitboard::{rf_to_square, set_bit};
use std::collections::HashMap;

pub fn generate_and_print_bitboard_constants() {
    let mut map = HashMap::new();

    let board_empty = 0;

    map.insert("BITBOARD_EMPTY".to_owned(), board_empty);

    let mut board_full = 0;

    for rank in 0..8 {
        for file in 0..8 {
            set_bit(&mut board_full, rf_to_square(rank, file))
        }
    }

    map.insert("BITBOARD_FULL".to_owned(), board_full);

    for rank in (0..8).rev() {
        let mut board = 0;

        for file in 0..8 {
            set_bit(&mut board, rf_to_square(rank, file));
        }

        map.insert(format!("RANK_{}", 8 - rank), board);
    }

    for file in 0..8 {
        let mut board = 0;

        for rank in 0..8 {
            set_bit(&mut board, rf_to_square(rank, file));
        }

        let character = match file {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            7 => "H",
            _ => unreachable!(),
        };

        map.insert(format!("FILE_{}", character), board);
    }

    map.insert("NOT_FILE_A".to_owned(), !(*map.get("FILE_A").unwrap()));
    map.insert("NOT_FILE_B".to_owned(), !(*map.get("FILE_B").unwrap()));
    map.insert("NOT_FILE_C".to_owned(), !(*map.get("FILE_C").unwrap()));
    map.insert("NOT_FILE_D".to_owned(), !(*map.get("FILE_D").unwrap()));
    map.insert("NOT_FILE_E".to_owned(), !(*map.get("FILE_E").unwrap()));
    map.insert("NOT_FILE_F".to_owned(), !(*map.get("FILE_F").unwrap()));
    map.insert("NOT_FILE_G".to_owned(), !(*map.get("FILE_G").unwrap()));
    map.insert("NOT_FILE_H".to_owned(), !(*map.get("FILE_H").unwrap()));
    map.insert("NOT_RANK_1".to_owned(), !(*map.get("RANK_1").unwrap()));
    map.insert("NOT_RANK_2".to_owned(), !(*map.get("RANK_2").unwrap()));
    map.insert("NOT_RANK_3".to_owned(), !(*map.get("RANK_3").unwrap()));
    map.insert("NOT_RANK_4".to_owned(), !(*map.get("RANK_4").unwrap()));
    map.insert("NOT_RANK_5".to_owned(), !(*map.get("RANK_5").unwrap()));
    map.insert("NOT_RANK_6".to_owned(), !(*map.get("RANK_6").unwrap()));
    map.insert("NOT_RANK_7".to_owned(), !(*map.get("RANK_7").unwrap()));
    map.insert("NOT_RANK_8".to_owned(), !(*map.get("RANK_8").unwrap()));

    map.insert(
        "NOT_FILE_AB".to_owned(),
        !(*map.get("FILE_A").unwrap() | *map.get("FILE_B").unwrap()),
    );
    map.insert(
        "NOT_FILE_GH".to_owned(),
        !(*map.get("FILE_G").unwrap() | *map.get("FILE_H").unwrap()),
    );

    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    for (k, v) in entries {
        println!("pub const {}: u64 = {};", k, v);
    }
}
