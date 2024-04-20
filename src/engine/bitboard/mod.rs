use std::fmt::Display;

mod square;

pub use square::*;

pub struct Bitboard(u64);

/*
    a   b   c   d   e   f   g   h
  ┌───┬───┬───┬───┬───┬───┬───┬───┐
8 │ 0 │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
7 │ 8 │ 9 │ 10│ 11│ 12│ 13│ 14│ 15│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
6 │ 16│ 17│ 18│ 19│ 20│ 21│ 22│ 23│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
5 │ 24│ 25│ 26│ 27│ 28│ 29│ 30│ 31│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
4 │ 32│ 33│ 34│ 35│ 36│ 37│ 38│ 39│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
3 │ 40│ 41│ 42│ 43│ 44│ 45│ 46│ 47│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
2 │ 48│ 49│ 50│ 51│ 52│ 53│ 54│ 55│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
1 │ 56│ 57│ 58│ 59│ 60│ 61│ 62│ 63│
  └───┴───┴───┴───┴───┴───┴───┴───┘
*/

impl Bitboard {
    pub fn empty() -> Self {
        Bitboard(0)
    }

    pub fn get(&self, index: u8) -> bool {
        (self.0 & (1 << index)) != 0
    }

    pub fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn clear(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    pub fn xy_to_index(x: u8, y: u8) -> u8 {
        x + y * 8
    }

    pub fn get_xy(&self, x: u8, y: u8) -> bool {
        self.get(Self::xy_to_index(x, y))
    }

    pub fn set_xy(&mut self, x: u8, y: u8) {
        self.set(Self::xy_to_index(x, y))
    }

    pub fn clear_xy(&mut self, x: u8, y: u8) {
        self.clear(Self::xy_to_index(x, y))
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  a b c d e f g h")?;

        for y in 0..8 {
            write!(f, "{} ", 8 - y)?;

            for x in 0..8 {
                if x != 0 {
                    write!(f, " ")?;
                }

                if self.get_xy(x, y) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }

            writeln!(f)?;
        }

        write!(f, "\ndec: {}", self.0)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_get_set_clear() {
        let mut bb = Bitboard::empty();

        assert!(!bb.get(0));
        assert!(!bb.get(1));
        assert!(!bb.get(63));

        bb.set(0);

        assert!(bb.get(0));
        assert!(!bb.get(1));
        assert!(!bb.get(63));

        bb.set(63);

        assert!(bb.get(0));
        assert!(!bb.get(1));
        assert!(bb.get(63));

        bb.clear(0);

        assert!(!bb.get(0));
        assert!(!bb.get(1));
        assert!(bb.get(63));

        bb.set(0);
        bb.clear(63);
        bb.clear(63);

        assert!(bb.get(0));
        assert!(!bb.get(1));
        assert!(!bb.get(63));
    }

    #[test]
    fn test_xy_to_index() {
        assert_eq!(Bitboard::xy_to_index(0, 0), 0);
        assert_eq!(Bitboard::xy_to_index(7, 7), 63);
        assert_eq!(Bitboard::xy_to_index(3, 4), 35);
    }

    #[test]
    fn test_bitboard_xy() {
        let mut bb = Bitboard::empty();

        assert!(!bb.get_xy(0, 0));
        assert!(!bb.get_xy(7, 7));

        bb.set_xy(0, 0);

        assert!(bb.get_xy(0, 0));
        assert!(!bb.get_xy(7, 7));

        bb.set_xy(7, 7);

        assert!(bb.get_xy(0, 0));
        assert!(bb.get_xy(7, 7));

        bb.clear_xy(0, 0);

        assert!(!bb.get_xy(0, 0));
        assert!(bb.get_xy(7, 7));

        bb.set_xy(0, 0);
        bb.clear_xy(7, 7);
        bb.clear_xy(7, 7);

        assert!(bb.get_xy(0, 0));
        assert!(!bb.get_xy(7, 7));
    }
}
