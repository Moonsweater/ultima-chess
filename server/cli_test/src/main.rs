use game::{
    move_validation,
    GameBoard,
    Rankfile,
    MoveData,
    UltimaPiece,
    UltimaPieceType,
    PlayerColor
};

fn print_board(board: &GameBoard) {

    fn square_to_char(s: Option<UltimaPiece>) -> String {
        let p = match s {
            None => return String::from("__ "),
            Some(p) => p
        };
        let mut out = String::from(match p.color {
            PlayerColor::Black => "b",
            PlayerColor::White => "w"
        });
        match p.piece_type {
            UltimaPieceType::Chameleon => {out += "C"},
            UltimaPieceType::Coordinator => {out += "O"},
            UltimaPieceType::Immobilizer => {out += "I"},
            UltimaPieceType::King => {out += "K"},
            UltimaPieceType::Longleaper => {out += "L"},
            UltimaPieceType::Pawn => {out += "P"},
            UltimaPieceType::Withdrawer => {out += "W"}
        }
        out + " "
    }

    //CODE:
    //K = king
    //W = withdrawer
    //I = immobilizer
    //C = chameleon
    //O = coordinator
    //L = longleaper
    //P = pawn

    //precede with b / w for color.
    //empty space is an underscore.

    let mut out = String::with_capacity(64 * 3 + 16);

    for r in (0..8).rev() {
        for f in 0..8 {
            out = out + square_to_char(board.get_square_from_coords(r, f)).as_str();
        }
        out = out + "\n\n";
    } 
    print!("{out}");
}

fn main() {
    let board = GameBoard::new_in_start_position();
    print_board(&board);
}

//Immediate bugfixes:

//Figure out why the board seems to not be flipped, even though it should be?
//Fix initial positions: