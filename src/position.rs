#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        return Self { row, col };
    }

    pub fn from_chess(string: &str) -> Option<Self> {
        if string.len() != 2 {
            return None;
        }

        let mut chars_iter = string.chars();

        let (col_chess, row_chess) = (chars_iter.next().unwrap(), chars_iter.next().unwrap());

        if !('a'..='h').contains(&col_chess) || !('1'..='8').contains(&row_chess) {
            return None;
        }

        let col = (col_chess as usize) - ('a' as usize);
        let row = 7 - ((row_chess as usize) - ('1' as usize));

        return Some(Self { row, col });
    }

    pub fn to_chess(&self) -> Option<String> {
        let row = self.row;
        let col = self.col;
        if row > 7 || col > 7 {
            return None;
        }

        let col_chess = ["a", "b", "c", "d", "e", "f", "g", "h"][col];
        let row_chess = 8 - row;

        return Some(format!("{}{}", col_chess, row_chess));
    }

    pub fn shift(&self, row_shift: isize, col_shift: isize) -> Option<Self> {
        let row = self.row as isize + row_shift;
        let col = self.col as isize + col_shift;

        if row < 0 || row > 7 || col < 0 || col > 7 {
            return None;
        }

        return Some(Self {
            row: row as usize,
            col: col as usize,
        });
    }
}
