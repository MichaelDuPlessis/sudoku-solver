use super::{Pos, State, board::Board};

pub struct Solver {
    game: Board,
}

impl Solver {
    pub fn new(game: Board) -> Self {
        Self {
            game,
        }
    }

    pub fn solve(&mut self) -> Option<&Board> {
        if self.check_placement((0, 0)) == State::Win {
            return Some(&self.game);
        }

        None
    }

    fn check_placement(&mut self, pos: Pos) -> State {
        // println!("{}, {}", pos.0, pos.1);
        // println!("{}", self.game);

        if self.game.is_piece(pos) {
            let (mut x, mut y) = pos;
            x += 1;
            if x == 9 {
                x = 0;
                y += 1;
            }

            return self.check_placement((x, y));
        }

        for p in 1..10 as u8 {
            match self.game.place_piece(p, pos) {
                Ok(s) => {
                    if s == State::Win {
                        return State::Win;
                    }

                    let (mut x, mut y) = pos;
                    x += 1;
                    if x == 9 {
                        x = 0;
                        y += 1;
                    }

                    if self.check_placement((x, y)) == State::Win {
                        return State::Win;
                    }
                },
                Err(_) => ()
            };

            match self.game.remove_piece(pos) {
                Ok(_) => (),
                Err(_) => ()
            };
        }
        
        State::NoWin
    }
}
