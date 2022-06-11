use std::{ops::Index, array::{IntoIter, self}};

use super::{PlaceResult, BoardErr, Pos, CheckResult, State};

// constants
const BOARD_SIZE: usize = 81;

pub struct Game {
    board: Board,
    grid: [bool; BOARD_SIZE],
    piece_count: u8, // u8 because size should never exceed 81
}

impl Game {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            grid: [false; BOARD_SIZE],
            piece_count: 0,
        }
    }

    pub fn place_piece(&mut self, piece: u8, pos: Pos) -> PlaceResult {
        // check if bounds are satified
        if pos.0 > 8 || pos.1 > 8 { // since pos must be of type (usize, usize) no need to check if < 0
            return Err(BoardErr::OutOfBounds);
        }
        if piece == 0 || piece > 9 {
            return Err(BoardErr::InvalidPiece);
        }

        // check if piece alreay existss
        if let Some(_) = self.board[pos.0 + pos.1 * 9] {
            return Err(BoardErr::PosTaken);
        }
        // code to check if placement valid
        self.check_placement(piece, pos)?;

        self.board[pos.0 + pos.1 * 9] = Some(piece);
        self.piece_count += 1;
        self.grid[(pos.0/3 + pos.1/3 * 3) * 9 + (piece - 1) as usize] = true;

        if self.piece_count as usize == BOARD_SIZE {
            return Ok(State::Win);
        }

        Ok(State::NoWin)
    }

    fn check_placement(&self, piece: u8, pos: Pos) -> CheckResult { // returns InvalidPlaceErr::PosInvalid
        // same grid cell
        if self.grid[(pos.0/3 + pos.1/3 * 3) * 9 + (piece - 1) as usize] {
            return Err(BoardErr::PosInvalid);
        }

        // checking row and col
        for i in 0..9 {
            if i != pos.0 {
                if let Some(p) = self.board[i + pos.1*9] {
                    if p == piece {
                        return Err(BoardErr::PosInvalid);
                    }
                }
            }
            if i != pos.1 {
                if let Some(p) = self.board[pos.0 + i*9] {
                    if p == piece {
                        return Err(BoardErr::PosInvalid);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn remove_piece(&mut self, pos: Pos) -> CheckResult {
        // check if bounds are satified
        if pos.0 > 8 || pos.1 > 8 { // since pos must be of type (usize, usize) no need to check if < 0
            return Err(BoardErr::OutOfBounds);
        }

        // take because if no piece then it leaves none otherweise remove piece
        match self.board[pos.0 + pos.1 * 9].take() {
            Some(p) =>  {
                self.grid[(pos.0/3 + pos.1/3 * 3) * 9 + (p - 1) as usize] = false;
                self.piece_count -= 1;
                Ok(())
            },
            None => Err(BoardErr::NoPiece),
        }
    }

    pub fn is_piece(&self, pos: Pos) -> bool {
        // remember error checking
        if self.board[pos.0 + pos.1 * 9] == None {
            return false;
        }

        true
    }
}

impl From<&[Option<u8>; 81]> for Game {
    fn from(board: &[Option<u8>; 81]) -> Self {
        let mut grid = [false; BOARD_SIZE];

        // change code to be more effiecient with modulus
        let mut x = 0;
        let mut y = 0;
        for piece in board {
            if let Some(p) = piece {
                grid[(x/3 + y/3 * 3) * 9 + (*p as usize) - 1] = true;
            }

            x += 1;
            if x == 9 {
                x = 0;
                y += 1;
            }
        }

        Self {
            board: Board::from(board), // maybe change so that Board holds a &[Option<u8>; 81]
            grid,
            piece_count: board.iter().filter(|p| { // create new data structure where None's are filtered out and count it
                **p != None
            }).count() as u8,
        }
    }
}

impl From<&str> for Game {
    fn from(board: &str) -> Self {
        Self::from(&board.as_bytes().iter().map(|p| {
            match p {
                b'1' => Some(1 as u8),
                b'2' => Some(2),
                b'3' => Some(3),
                b'4' => Some(4),
                b'5' => Some(5),
                b'6' => Some(6),
                b'7' => Some(7),
                b'8' => Some(8),
                b'9' => Some(9),
                _ => None,
              }
        }).collect::<Board>())
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pretty_board = String::new();
        let mut pos = 0;
        let mut line = 0;

        for piece in self.board {
            if pos % 3 == 0 && pos != 0 {
                pretty_board.push('|');
            }

            if let Some(p) = piece {
                pretty_board.push_str(&p.to_string());
            } else {
                pretty_board.push('0');
            }

            pos += 1;
            if pos == 9 {
                pos = 0;
                line += 1;
                pretty_board.push('\n');

                if line % 3 == 0 && line != 9 {
                    pretty_board.push_str("-----------\n");
                }
            }
        }

        write!(f, "{}", pretty_board)
    }
}

struct Board {
    board: [Option<u8>; BOARD_SIZE]
}

impl Board {
    fn new() -> Self {
        Self {
            board: [None; BOARD_SIZE]
        }
    }
}

impl FromIterator<Option<u8>> for Board {
    fn from_iter<T: IntoIterator<Item = Option<u8>>>(iter: T) -> Self {
        let board = Board::new();

        for (i, b) in iter.into_iter().enumerate() {
            board[i] = b;
        }

        board
    }
}

impl IntoIterator for Board {
    type Item = Option<u8>;
    type IntoIter = array::IntoIter<Self::Item, BOARD_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.board.into_iter()
    }
}

impl From<&[Option<u8>; BOARD_SIZE]> for Board {
    fn from(board: &[Option<u8>; BOARD_SIZE]) -> Self {
        Self {
            board: *board
        }
    }
}

impl Index<usize> for Board {
    type Output = Option<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

// unwrap is used because I know that there will always be an Ok()
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_place_test() {
        assert_eq!(Game::new().place_piece(7, (0, 7)), Ok(State::NoWin));

        // add code to test for win
    }

    #[test]
    fn invalid_place_test() {
        let mut board = Game::new();
        board.place_piece(1, (0, 7)).unwrap();
        assert_eq!(board.place_piece(7, (0, 7)), Err(BoardErr::PosTaken));

        let mut board = Game::new();
        board.place_piece(7, (0, 0)).unwrap();
        assert_eq!(board.place_piece(7, (0, 7)), Err(BoardErr::PosInvalid));

        assert_eq!(Game::new().place_piece(7, (0, 20)), Err(BoardErr::OutOfBounds));

        assert_eq!(Game::new().place_piece(20, (0, 0)), Err(BoardErr::InvalidPiece));
    }

    #[test]
    fn valid_remove_test() {
        let mut board = Game::new();
        board.place_piece(1, (0, 7)).unwrap();
        assert_eq!(board.remove_piece((0,7)), Ok(()))
    }

    #[test]
    fn invalid_remove_test() {
        let mut board = Game::new();
        board.place_piece(1, (0, 7)).unwrap();
        assert_eq!(board.remove_piece((0,6)), Err(BoardErr::NoPiece));
    }

    #[test]
    fn check_grid() {
        let mut board = Game::new();
        board.place_piece(1, (0, 0)).unwrap();
        board.place_piece(2, (1, 0)).unwrap();
        board.place_piece(3, (2, 0)).unwrap();
        board.place_piece(4, (0, 1)).unwrap();
        board.place_piece(5, (0, 2)).unwrap();
        board.place_piece(6, (1, 1)).unwrap();
        board.place_piece(7, (2, 2)).unwrap();
        board.place_piece(8, (1, 2)).unwrap();
        assert_eq!(board.place_piece(9, (2, 1)), Ok(State::NoWin));
        println!("{:?}", board.grid);
    }
}