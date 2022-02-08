// constants
const BOARD_SIZE: usize = 81;

// enums
#[derive(Debug, PartialEq, Eq)]
enum BoardErr {
    PosInvalid, // breaks rules
    PosTaken, // already a piece
    NoPiece, // no piece to remove
    OutOfBounds, // bounds violated
    InvalidPiece, // if piece is invalid
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Win,
    NoWin,
}

// types
type Pos = (usize, usize);
type PlaceResult = Result<State, BoardErr>;
type CheckResult = Result<(), BoardErr>;

pub struct Board {
    board: [Option<u8>; BOARD_SIZE],
    grid: [bool; BOARD_SIZE],
    piece_count: u8,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [None; BOARD_SIZE],
            grid: [false; BOARD_SIZE],
            piece_count: 0,
        }
    }

    // add code to check if board is valid
    // make it so gird does not need to be past in
    pub fn from_array(board: [Option<u8>; BOARD_SIZE]) -> Self {
        let mut grid = [false; BOARD_SIZE];

        // change code to be more effiecient with modulus
        let mut x = 0;
        let mut y = 0;
        for piece in board {
            if let Some(p) = piece {
                grid[x/3 + y/3 * 3 + (p as usize) - 1] = true;
            }

            x += 1;
            if x == 9 {
                x = 0;
                y += 1;
            }
        }

        Self {
            board,
            grid,
            piece_count: board.iter().filter(|p| { // create new data structure where None's are filtered out and count it
                **p != None
            }).count() as u8,
        }
    }

    pub fn from_string(board: &str) -> Self {
        todo!()
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
        if let Some(p) = self.board[pos.0 + pos.1 * 9] {
            return Err(BoardErr::PosTaken);
        }
        // code to check if placement valid
        self.check_placement(piece, pos)?;

        self.board[pos.0 + pos.1 * 9] = Some(piece);
        self.piece_count += 1;
        self.grid[pos.0/3 + pos.1/3 * 3 + (piece - 1) as usize] = true;

        if self.piece_count as usize == BOARD_SIZE {
            return Ok(State::Win);
        }

        Ok(State::NoWin)
    }

    fn check_placement(&self, piece: u8, pos: Pos) -> CheckResult { // returns InvalidPlaceErr::PosInvalid
        // same grid cell
        if self.grid[pos.0/3 + pos.1/3 * 3 + (piece - 1) as usize] {
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
                self.grid[pos.0/3 + pos.1/3 * 3 + (p - 1) as usize] = false;
                self.piece_count -= 1;
                Ok(())
            },
            None => Err(BoardErr::NoPiece),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pretty_board = String::new();
        let mut pos = 0;
        let mut line = 0;

        for piece in &self.board {
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

// unwrap is used because I know that there will always be an Ok()
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_place_test() {
        assert_eq!(Board::new().place_piece(7, (0, 7)), Ok(State::NoWin));

        // add code to test for win
    }

    #[test]
    fn invalid_place_test() {
        let mut board = Board::new();
        board.place_piece(1, (0, 7)).unwrap();
        assert_eq!(board.place_piece(7, (0, 7)), Err(BoardErr::PosTaken));

        let mut board = Board::new();
        board.place_piece(7, (0, 0)).unwrap();
        assert_eq!(board.place_piece(7, (0, 7)), Err(BoardErr::PosInvalid));

        assert_eq!(Board::new().place_piece(7, (0, 20)), Err(BoardErr::OutOfBounds));

        assert_eq!(Board::new().place_piece(20, (0, 0)), Err(BoardErr::InvalidPiece));
    }

    #[test]
    fn valid_remove_test() {
        let mut board = Board::new();
        board.place_piece(1, (0, 7)).unwrap();
        assert_eq!(board.remove_piece((0,7)), Ok(()))
    }

    #[test]
    fn invalid_remove_test() {
        let mut board = Board::new();
        board.place_piece(1, (0, 7)).unwrap();
        assert_eq!(board.remove_piece((0,6)), Err(BoardErr::NoPiece));
    }

    #[test]
    fn check_grid() {
        let mut board = Board::new();
        board.place_piece(1, (0, 0)).unwrap();
        board.place_piece(2, (0, 1)).unwrap();
        board.place_piece(3, (0, 2)).unwrap();
        board.place_piece(4, (1, 3)).unwrap();
        board.place_piece(5, (1, 4)).unwrap();
        board.place_piece(6, (1, 5)).unwrap();
        board.place_piece(7, (2, 6)).unwrap();
        board.place_piece(8, (2, 7)).unwrap();
        assert_eq!(board.place_piece(9, (2, 8)), Ok(State::NoWin));
    }
}