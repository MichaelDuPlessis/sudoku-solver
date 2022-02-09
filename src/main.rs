mod game;
use game::{solver::Solver, board::Board};

fn main() {
    let board = "100240000240805903000701204409158006701009040526000090012500000000037162600000508";
    let mut solver = Solver::new(Board::new());
    println!("{}", solver.solve().unwrap());
}
