use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Default)]
pub struct Checkers {
    turn: Color,
    just_take: Option<(usize, usize)>,
    board: Board,
}

impl Checkers {
    pub fn is_black_turn(&self) -> bool {
        self.turn == Color::Black
    }
    pub fn try_move_piece(&mut self, msg: &str) -> Result<(), ()> {
        if msg == "skip" {
            if self.just_take.is_none() {
                return Err(());
            }
            self.turn = self.turn.other();
            self.just_take = None;
            return Ok(());
        }

        let Move {
            from_col,
            from_row,
            to_col,
            to_row,
        } = msg.parse().map_err(|_| ())?;

        let col_diff = usize::abs_diff(from_col, to_col);
        let row_diff = usize::abs_diff(from_row, to_row);

        match self.board.0[from_row][from_col] {
            // Invalid moves
            _ if col_diff != row_diff => return Err(()),
            _ if self.just_take.is_some()
                && (self.just_take.unwrap() != (from_row, from_col) || col_diff != 2) =>
            {
                return Err(())
            }
            Some(Piece { color, .. }) if color != self.turn => return Err(()),
            // King moves
            Some(Piece { king: true, .. }) if col_diff == 1 => {
                self.move_piece(from_col, from_row, to_col, to_row)
            }
            // King takes
            Some(Piece { king: true, color }) if col_diff == 2 => {
                self.take_piece(from_col, from_row, to_col, to_row, color)
            }
            // Black moves
            Some(Piece {
                color: Color::Black,
                ..
            }) if col_diff == 1 && to_row == from_row + 1 => {
                self.move_piece(from_col, from_row, to_col, to_row)
            }
            // Black takes
            Some(Piece {
                color: Color::Black,
                ..
            }) if col_diff == 2 && to_row == from_row + 2 => {
                self.take_piece(from_col, from_row, to_col, to_row, Color::Black)
            }
            // White moves
            Some(Piece {
                color: Color::White,
                ..
            }) if col_diff == 1 && from_row == to_row + 1 => {
                self.move_piece(from_col, from_row, to_col, to_row)
            }
            // White takes
            Some(Piece {
                color: Color::White,
                ..
            }) if col_diff == 2 && from_row == to_row + 2 => {
                self.take_piece(from_col, from_row, to_col, to_row, Color::White)
            }
            // Others
            _ => return Err(()),
        }
        // Queening
        if to_row == 0 || to_row == 7 {
            self.board.0[to_row][to_col] = Some(Piece {
                color: self.turn,
                king: true,
            });
        }

        Ok(())
    }
    fn move_piece(&mut self, from_col: usize, from_row: usize, to_col: usize, to_row: usize) {
        if self.board.0[to_row][to_col].is_none() {
            self.board.0[to_row][to_col] = self.board.0[from_row][from_col];
            self.board.0[from_row][from_col] = None;
            self.turn = self.turn.other();
        }
    }
    fn take_piece(
        &mut self,
        from_col: usize,
        from_row: usize,
        to_col: usize,
        to_row: usize,
        color: Color,
    ) {
        let mid_col = (from_col + to_col) / 2;
        let mid_row = (from_row + to_row) / 2;
        if let Some(target) = self.board.0[mid_row][mid_col] {
            if target.color != color && self.board.0[to_row][to_col].is_none() {
                self.board.0[to_row][to_col] = self.board.0[from_row][from_col];
                self.board.0[from_row][from_col] = None;
                self.board.0[mid_row][mid_col] = None;
                self.just_take = Some((to_row, to_col));
            }
        }
    }
}

impl ToString for Checkers {
    fn to_string(&self) -> String {
        if let Some((row, col)) = self.just_take {
            format!(
                "{{ \"turn\": \"{}\", \"just_take\": [{}, {}], \"board\": \"{}\" }}",
                self.turn,
                row,
                col,
                self.board.to_string().replace("\n", "")
            )
        } else {
            format!(
                "{{ \"turn\": \"{}\", \"board\": \"{}\" }}",
                self.turn,
                self.board.to_string().replace("\n", "")
            )
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    fn other(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
        }
    }
}

#[derive(Clone, Copy)]
struct Piece {
    color: Color,
    king: bool,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Color::*;
        match (self.color, self.king) {
            (White, false) => write!(f, "⚪")?,
            (White, true) => write!(f, "⬜")?,
            (Black, false) => write!(f, "⚫")?,
            (Black, true) => write!(f, "⬛")?,
        }
        Ok(())
    }
}

impl From<Color> for Option<Piece> {
    fn from(color: Color) -> Self {
        Self::Some(Piece { color, king: false })
    }
}

struct Board([[Option<Piece>; 8]; 8]);

impl Default for Board {
    fn default() -> Self {
        use Color::*;
        let mut s = Self([[None; 8]; 8]);
        for i in 0..4 {
            s.0[0][i * 2 + 1] = Black.into();
            s.0[1][i * 2] = Black.into();
            s.0[2][i * 2 + 1] = Black.into();
            s.0[5][i * 2] = White.into();
            s.0[6][i * 2 + 1] = White.into();
            s.0[7][i * 2] = White.into();
        }
        s
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for piece in row.iter() {
                match piece {
                    Some(piece) => write!(f, "{}", piece)?,
                    None => write!(f, "🟦")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Move {
    from_col: usize,
    from_row: usize,
    to_col: usize,
    to_row: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [from_row, from_col, to_row, to_col] = s
            .split(',')
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ())?[..]
        {
            if from_col < 8 && from_row < 8 && to_col < 8 && to_row < 8 {
                Ok(Self {
                    from_col,
                    from_row,
                    to_col,
                    to_row,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
