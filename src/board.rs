// constants
const BOARD_SIZE: usize = 81;

// enums
enum BoardErr {
    PosInvalid, // breaks rules
    PosTaken, // already a piece
    NoPiece, // no piece to remove
    OutOfBounds, // bounds violated
}

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
    piece_count: u8,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [None; BOARD_SIZE],
            piece_count: 0,
        }
    }

    pub fn from_array(board: [Option<u8>; BOARD_SIZE]) -> Self {
        Self {
            board,
            piece_count: board.iter().filter(|p| {
                **p != None
            }).count() as u8,
        }
    }

    pub fn place_piece(&mut self, piece: u8, pos: Pos) -> PlaceResult {
        // check if bounds are satified
        if pos.0 > 8 || pos.1 > 0 { // since pos must be of type (usize, usize) no need to check if < 0
            return Err(BoardErr::OutOfBounds);
        }

        // check if piece alreay existss
        if let Some(p) = self.board[pos.0 + pos.1 * 9] {
            return Err(BoardErr::PosTaken);
        }
        // code to check if placement valid
        self.check_placement(piece, pos)?;

        self.board[pos.0 + pos.1 * 9] = Some(piece);
        self.piece_count += 1;

        if self.piece_count as usize == BOARD_SIZE {
            return Ok(State::Win);
        }

        Ok(State::NoWin)
    }

    fn check_placement(&self, piece: u8, pos: Pos) -> CheckResult { // returns InvalidPlaceErr::PosInvalid
        // same grid cell
        let grid: [i8; 8] = [-1, 1, -8, 8, -9, 9, -10, 10];
        for g in grid {
            let cell = (pos.0 as i8) + (pos.1 as i8)*9 + g;
            if cell >= 0 && cell < 81 {
                if let Some(p) = self.board[cell as usize] {
                    if p == piece {
                        return Err(BoardErr::PosInvalid);
                    }
                }
            }
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
        if pos.0 > 8 || pos.1 > 0 { // since pos must be of type (usize, usize) no need to check if < 0
            return Err(BoardErr::OutOfBounds);
        }

        // take because if no piece then it leaves none otherweise remove piece
        match self.board[pos.0 + pos.1 * 9].take() {
            Some(_) => Ok(()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_print_test() {
        let mut board = Board::new();
        board.place_piece(1, (1, 0));
        board.place_piece(1, (2, 0));
        println!("{}", board);
    }
}