// use std::fmt::Display;

use actix_web::{get, web::Path, HttpResponse};

#[derive(PartialEq, Clone, Copy)]
enum Square {
    Empty,
    X,
    O,
}

// impl Display for Square {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Square::Empty => "-",
//                 Square::X => "X",
//                 Square::O => "O",
//             }
//         )
//     }
// }

#[derive(Clone, Copy)]
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
    fn from_string(s: &str) -> Board {
        let mut squares = [Square::Empty; 9];
        for (i, c) in s.chars().enumerate() {
            squares[i] = match c {
                'X' | 'x' => Square::X,
                'O' | 'o' => Square::O,
                _ => Square::Empty,
            };
        }
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

    fn get_available_moves(&self, turn: &Turn) -> Vec<Board> {
        let mut moves = Vec::with_capacity(9);
        for (i, square) in self.squares.iter().enumerate() {
            if *square == Square::Empty {
                let mut squares = self.squares;
                squares[i] = match turn {
                    Turn::X => Square::X,
                    Turn::O => Square::O,
                };
                moves.push(Board::from_squares(squares));
            }
        }
        moves
    }

    fn minimax(&self, turn: &Turn) -> Option<(GameState, Board)> {
        return match self.score() {
            GameState::OHasWon => Some((GameState::OHasWon, *self)),
            GameState::XHasWon => Some((GameState::XHasWon, *self)),
            GameState::Tie => Some((GameState::Tie, *self)),
            GameState::NotFinished => {
                let mut optional_best_move: Option<(GameState, Board)> = None;
                for possible_move in self.get_available_moves(&turn) {
                    if let Some((game_state, _board)) = possible_move.minimax(&turn.next()) {
                        // if this move is better than the current best move, then set it
                        if let Some(best_move) = &optional_best_move {
                            if *turn == Turn::X {
                                if game_state.partial_cmp(&best_move.0)
                                    == Some(std::cmp::Ordering::Greater)
                                {
                                    optional_best_move = Some((game_state, possible_move));
                                }
                            } else {
                                if game_state.partial_cmp(&best_move.0)
                                    == Some(std::cmp::Ordering::Less)
                                {
                                    optional_best_move = Some((game_state, possible_move));
                                }
                            }
                        } else {
                            optional_best_move = Some((game_state, possible_move));
                        }
                    }
                }

                optional_best_move
            }
        };
    }
}

// impl Display for Board {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for i in 0..9 {
//             write!(f, "{} ", self.squares[i])?;
//             if i % 3 == 2 {
//                 write!(f, "\n")?;
//             }
//         }
//         Ok(())
//     }
// }

#[derive(PartialEq, Debug, Clone, Copy)]
enum GameState {
    NotFinished,
    XHasWon,
    OHasWon,
    Tie,
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (GameState::XHasWon, GameState::XHasWon) => Some(std::cmp::Ordering::Equal),
            (GameState::XHasWon, _) => Some(std::cmp::Ordering::Greater),
            (_, GameState::XHasWon) => Some(std::cmp::Ordering::Less),
            (GameState::OHasWon, GameState::OHasWon) => Some(std::cmp::Ordering::Equal),
            (GameState::OHasWon, _) => Some(std::cmp::Ordering::Less),
            (_, GameState::OHasWon) => Some(std::cmp::Ordering::Greater),
            (GameState::Tie, GameState::Tie) => Some(std::cmp::Ordering::Equal),
            (GameState::NotFinished, GameState::NotFinished) => Some(std::cmp::Ordering::Equal),
            (GameState::Tie, _) => Some(std::cmp::Ordering::Greater),
            (_, GameState::Tie) => Some(std::cmp::Ordering::Less),
        }
    }
}

#[derive(PartialEq)]
enum Turn {
    X,
    O,
}

impl Turn {
    fn next(&self) -> Turn {
        match self {
            Turn::O => Turn::X,
            Turn::X => Turn::O,
        }
    }
}

// fn main() {
//     let mut board = Board::new();
//     println!(
//         "Welcome to Tic Tac Toe!
// You are X, the computer is O.
// The board is numbered like this:
// 1 2 3
// 4 5 6
// 7 8 9
// Good luck!"
//     );
//     loop {
//         println!("{}", board);
//         println!("Enter a move (1-9):");
//         let mut input = String::new();
//         std::io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         let input: usize = match input.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Invalid input, must be a number");
//                 continue;
//             }
//         };
//         if input < 1 || input > 9 {
//             println!("Invalid input, must be between 1 and 9");
//             continue;
//         }
//         let input = input - 1;
//         if board.squares[input] != Square::Empty {
//             println!("Invalid input, square already taken");
//             continue;
//         }
//         board.squares[input] = Square::X;
//         if let Some((_game_state, best_move)) = board.minimax(&Turn::O) {
//             board = best_move;
//             if board.score() == GameState::OHasWon {
//                 println!("You lost!");
//                 break;
//             } else if board.score() == GameState::Tie {
//                 println!("It's a tie!");
//                 break;
//             }
//         } else {
//             unreachable!();
//         }
//     }
// }

#[get("/{board}/{turn}")]
pub async fn get_best_move(path: Path<(String, String)>) -> HttpResponse {
    if path.0.len() != 9 {
        return HttpResponse::BadRequest().body("Invalid board");
    }
    let board = Board::from_string(&path.0);
    let turn = match path.1.as_str() {
        "X" | "x" => Turn::X,
        "O" | "o" => Turn::O,
        _ => return HttpResponse::BadRequest().body("Invalid turn"),
    };

    let best_move = board.minimax(&turn).unwrap().1;

    HttpResponse::Ok().body(
        best_move
            .squares
            .iter()
            .fold("".to_string(), |acc, squares| {
                acc + match squares {
                    Square::Empty => "-",
                    Square::X => "X",
                    Square::O => "O",
                }
            }),
    )
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

    #[test]
    fn test_minimax() {
        let x_win_board = Board::from_squares([
            Square::X,
            Square::Empty,
            Square::Empty,
            Square::X,
            Square::O,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::O,
        ]);
        // x--
        // xo-
        // --o
        assert_eq!(x_win_board.minimax(&Turn::X).unwrap().0, GameState::XHasWon);
        let o_win_board = Board::from_squares([
            Square::X,
            Square::Empty,
            Square::O,
            Square::X,
            Square::Empty,
            Square::Empty,
            Square::O,
            Square::Empty,
            Square::X,
        ]);
        // x-o
        // x--
        // o-x
        assert_eq!(o_win_board.minimax(&Turn::O).unwrap().0, GameState::OHasWon);
        let tie_board = Board::from_squares([
            Square::X,
            Square::O,
            Square::Empty,
            Square::X,
            Square::O,
            Square::O,
            Square::O,
            Square::X,
            Square::X,
        ]);
        // xo-
        // xoo
        // oxx
        assert_eq!(tie_board.minimax(&Turn::X).unwrap().0, GameState::Tie);
        assert_eq!(Board::new().minimax(&Turn::X).unwrap().0, GameState::Tie);
    }
}
