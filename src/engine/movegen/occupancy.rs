use crate::engine::bitboard::Bitboard;

pub fn mask_occupancy(attack_mask: Bitboard, mut index: usize) -> Bitboard {
    let mut occupancy = Bitboard::empty();

    for square in attack_mask.squares() {
        if index & 1 != 0 {
            occupancy.set(square);
        }

        index >>= 1;
    }

    occupancy
}

#[cfg(test)]
mod tests {
    use crate::engine::bitboard::Square;

    use super::*;

    #[test]
    fn test_mask_occupancy() {
        // rook attack mask on A1
        let mask = Square::A7.to_bb()
            | Square::A6.to_bb()
            | Square::A5.to_bb()
            | Square::A4.to_bb()
            | Square::A3.to_bb()
            | Square::A2.to_bb()
            | Square::B1.to_bb()
            | Square::C1.to_bb()
            | Square::D1.to_bb()
            | Square::E1.to_bb()
            | Square::F1.to_bb()
            | Square::G1.to_bb();
        let bits_in_mask = mask.count_ones();
        let max_index = (1usize << bits_in_mask) - 1;

        let occupancy = mask_occupancy(mask, 0);
        assert_eq!(occupancy, Bitboard::empty());

        let occupancy = mask_occupancy(mask, 1);
        assert_eq!(occupancy, Square::A7.to_bb());

        let occupancy = mask_occupancy(mask, 7);
        assert_eq!(
            occupancy,
            Square::A7.to_bb() | Square::A6.to_bb() | Square::A5.to_bb()
        );

        let occupancy = mask_occupancy(mask, max_index);
        assert_eq!(
            occupancy,
            Square::A7.to_bb()
                | Square::A6.to_bb()
                | Square::A5.to_bb()
                | Square::A4.to_bb()
                | Square::A3.to_bb()
                | Square::A2.to_bb()
                | Square::B1.to_bb()
                | Square::C1.to_bb()
                | Square::D1.to_bb()
                | Square::E1.to_bb()
                | Square::F1.to_bb()
                | Square::G1.to_bb()
        );
    }
}
