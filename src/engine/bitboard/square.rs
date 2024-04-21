use strum::FromRepr;

#[allow(dead_code)]
#[repr(u8)]
#[derive(FromRepr, Debug, PartialEq, Clone, Copy)]
pub enum Square {
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
}

impl Square {
    pub fn to_repr(self) -> u8 {
        self as u8
    }

    pub fn to_bb(self) -> u64 {
        1 << self.to_repr()
    }
}

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;

pub const NOT_FILE_A: u64 = !FILE_A;
pub const NOT_FILE_H: u64 = !FILE_H;
pub const NOT_FILE_AB: u64 = !(FILE_A | FILE_B);
pub const NOT_FILE_GH: u64 = !(FILE_G | FILE_H);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_to_repr() {
        assert_eq!(Square::A8.to_repr(), 0);
        assert_eq!(Square::H8.to_repr(), 7);
        assert_eq!(Square::A1.to_repr(), 56);
        assert_eq!(Square::H1.to_repr(), 63);
    }

    #[test]
    fn test_square_from_repr() {
        assert_eq!(Square::from_repr(0), Some(Square::A8));
        assert_eq!(Square::from_repr(7), Some(Square::H8));
        assert_eq!(Square::from_repr(56), Some(Square::A1));
        assert_eq!(Square::from_repr(63), Some(Square::H1));
    }

    #[test]
    fn test_file_masks() {
        assert_eq!(FILE_A & Square::A8.to_bb(), Square::A8.to_bb());
        assert_eq!(FILE_A & Square::B8.to_bb(), 0);
        assert_eq!(FILE_A & FILE_B, 0);

        assert_eq!(FILE_H & Square::H8.to_bb(), Square::H8.to_bb());
        assert_eq!(FILE_H & Square::G8.to_bb(), 0);
        assert_eq!(FILE_H & FILE_G, 0);

        assert_eq!(NOT_FILE_A & Square::A8.to_bb(), 0);
        assert_eq!(NOT_FILE_A & Square::B8.to_bb(), Square::B8.to_bb());
        assert_eq!(NOT_FILE_A & FILE_B, FILE_B);

        assert_eq!(NOT_FILE_H & Square::H8.to_bb(), 0);
        assert_eq!(NOT_FILE_H & Square::G8.to_bb(), Square::G8.to_bb());
        assert_eq!(NOT_FILE_H & FILE_G, FILE_G);

        assert_eq!(NOT_FILE_AB & Square::A8.to_bb(), 0);
        assert_eq!(NOT_FILE_AB & Square::B8.to_bb(), 0);
        assert_eq!(NOT_FILE_AB & Square::C8.to_bb(), Square::C8.to_bb());

        assert_eq!(NOT_FILE_GH & Square::H8.to_bb(), 0);
        assert_eq!(NOT_FILE_GH & Square::G8.to_bb(), 0);
        assert_eq!(NOT_FILE_GH & Square::F8.to_bb(), Square::F8.to_bb());
    }
}
