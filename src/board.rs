// constants
const BOARD_SIZE: usize = 81;

// enums
enum InvalidPlaceErr {
    PosInvalid, // breaks rules
    PosTaken // already a piece
}

// types
type Pos = (usize, usize);

pub struct Board {
    board: [Option<u8>; BOARD_SIZE]
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [None; BOARD_SIZE]
        }
    }

    pub fn from_array(board: [Option<u8>; BOARD_SIZE]) -> Self {
        Self {
            board
        }
    }

    pub fn place_piece(&mut self, piece: u8, pos: Pos) -> Result<(), InvalidPlaceErr> {
        if let Some(p) = self.board[pos.0 + pos.1 * 9] {
            return Err(InvalidPlaceErr::PosTaken);
        } else {
            // code to check if placement valid
            self.check_placement(piece, pos)?;
        }

        self.board[pos.0 + pos.1 * 9] = Some(piece);

        Ok(())
    }

    fn check_placement(&self, piece: u8, pos: Pos) -> Result<(), InvalidPlaceErr> { // returns InvalidPlaceErr::PosInvalid

        Ok(())
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
                pretty_board.push(' ');
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
        let board = Board::from_array([Some(0); 81]);
        println!("{}", board);
    }
}