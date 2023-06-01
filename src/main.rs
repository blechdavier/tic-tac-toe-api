use std::fmt::Display;

#[derive(PartialEq)]
enum Square {
    Empty,
    X,
    O,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::Empty => "-",
                Square::X => "X",
                Square::O => "O",
            }
        )
    }
}

struct Board {
    /// Reads like English, left to right, top to bottom
    squares: [Square; 9],
}

impl Board {
    fn new() -> Board {
        const EMPTY_SQUARE: Square = Square::Empty;
        Board {
            squares: [EMPTY_SQUARE; 9],
        }
    }
    fn from_squares(squares: [Square; 9]) -> Board {
        Board { squares }
    }
    fn score(&self) -> GameState {
        // verticals
        for i in 0..3 {
            if self.squares[i] == self.squares[i + 3] && self.squares[i] == self.squares[i + 6] {
                match self.squares[i] {
                    Square::X => return GameState::XHasWon,
                    Square::O => return GameState::OHasWon,
                    Square::Empty => (),
                };
            }
        }

        // horizontals
        for i in 0..3 {
            if self.squares[i * 3] == self.squares[i * 3 + 1]
                && self.squares[i * 3] == self.squares[i * 3 + 2]
            {
                match self.squares[i * 3] {
                    Square::X => return GameState::XHasWon,
                    Square::O => return GameState::OHasWon,
                    Square::Empty => (),
                };
            }
        }

        // diagonals
        if self.squares[0] == self.squares[4] && self.squares[0] == self.squares[8] {
            match self.squares[0] {
                Square::X => return GameState::XHasWon,
                Square::O => return GameState::OHasWon,
                Square::Empty => (),
            };
        }

        if self.squares[2] == self.squares[4] && self.squares[2] == self.squares[6] {
            match self.squares[2] {
                Square::X => return GameState::XHasWon,
                Square::O => return GameState::OHasWon,
                Square::Empty => (),
            };
        }

        if self.squares.contains(&Square::Empty) {
            return GameState::NotFinished;
        }
        GameState::Tie
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            write!(f, "{}", self.squares[i])?;
            if i % 3 == 2 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
enum GameState {
    NotFinished,
    XHasWon,
    OHasWon,
    Tie,
}

fn main() {
    let mut board = Board::new();
    println!("{}", board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring() {
        assert_eq!(Board::new().score(), GameState::NotFinished);
        assert_eq!(
            Board::from_squares([
                Square::X,
                Square::Empty,
                Square::Empty,
                Square::X,
                Square::O,
                Square::Empty,
                Square::X,
                Square::Empty,
                Square::O,
            ])
            .score(),
            GameState::XHasWon
        );
        assert_eq!(
            Board::from_squares([
                Square::X,
                Square::Empty,
                Square::O,
                Square::X,
                Square::O,
                Square::Empty,
                Square::O,
                Square::Empty,
                Square::X,
            ])
            .score(),
            GameState::OHasWon
        );
        assert_eq!(
            Board::from_squares([
                Square::X,
                Square::O,
                Square::X,
                Square::X,
                Square::O,
                Square::O,
                Square::O,
                Square::X,
                Square::X,
            ])
            .score(),
            GameState::Tie
        );
    }
}
