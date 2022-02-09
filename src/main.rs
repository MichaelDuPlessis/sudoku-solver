mod game;
use game::{solver::Solver, board::Board};

fn main() {
    let board = "000002000080000007006310900060250080000004500002009000100000090000040000003560100";
    let mut solver = Solver::new(Board::from_string(board));
    println!("{}", solver.solve().unwrap());
}
