mod board;
mod solver;

fn main() {
    let mut solver = solver::Solver::new(board::Board::new());
    println!("{}", solver.solve().unwrap());
}
