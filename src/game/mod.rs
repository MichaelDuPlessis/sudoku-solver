pub mod board;
pub mod solver;

// enums
#[derive(Debug, PartialEq, Eq)]
pub enum BoardErr {
    PosInvalid, // breaks rules
    PosTaken, // already a piece
    NoPiece, // no piece to remove
    OutOfBounds, // bounds violated
    InvalidPiece, // if piece is invalid
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Win,
    NoWin,
}

// types
type Pos = (usize, usize);
type PlaceResult = Result<State, BoardErr>;
type CheckResult = Result<(), BoardErr>;