mod game;

fn main() {
    let board = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
    let mut solver = game::solver::Solver::new(game::board::Board::from_string(board));
    println!("{}", solver.solve().unwrap());
}
