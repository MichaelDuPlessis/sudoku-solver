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
        // same grid cell
        let grid: [i8; 8] = [-1, 1, -8, 8, -9, 9, -10, 10];
        for g in grid {
            let cell = (pos.0 as i8) + (pos.1 as i8)*9 + g;
            if cell >= 0 && cell < 81 {
                if let Some(p) = self.board[cell as usize] {
                    if p == piece {
                        return Err(InvalidPlaceErr::PosInvalid);
                    }
                }
            }
        }

        // checking row and col
        for i in 0..9 {
            if i != pos.0 {
                if let Some(p) = self.board[i + pos.1*9] {
                    if p == piece {
                        return Err(InvalidPlaceErr::PosInvalid);
                    }
                }
            }
            if i != pos.1 {
                if let Some(p) = self.board[pos.0 + i*9] {
                    if p == piece {
                        return Err(InvalidPlaceErr::PosInvalid);
                    }
                }
            }
        }

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
        let x = [-7];
        for i in x {
            println!("{}", i as usize);
        }
    }
}