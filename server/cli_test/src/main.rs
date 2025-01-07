use game::{
    move_validation,
    execute_move,
    GameBoard,
    Rankfile,
    //MoveData,
    UltimaPiece,
    UltimaPieceType,
    PlayerColor
};

fn board_to_string(board: &GameBoard) -> String {

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

    let mut out = String::with_capacity(64 * 3 + 4 * 8 + 4 * 8 + 4 * 4);

    for r in (0..8).rev() {
        out = out + format!("[{}] ", r + 1).as_str();
        for f in 0..8 {
            out = out + square_to_char(board.get_square_from_coords(r, f)).as_str();
        }
        out = out + "\n\n";
    }
    out = out + "   " + "[A]" + "[B]" + "[C]" + "[D]" + "[E]" + "[F]" + "[G]" + "[H]" + "\n\n";
    out
}

fn scan_string() -> String {
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();
    out
}

fn flip_color(color: PlayerColor) -> PlayerColor {
    match color {
        PlayerColor::Black => PlayerColor::White,
        PlayerColor::White => PlayerColor::Black
    }
}

fn main() {

    let mut board = GameBoard::new_in_start_position();

    let mut whose_turn = PlayerColor::White;
    loop {
        println!("{}", board_to_string(&board));
        let bw = match whose_turn {
            PlayerColor::Black => "black",
            PlayerColor::White => "white",
        };
        println!("{bw}'s turn.");
        let (piece_moved, start) = loop {
            println!("Which piece will you move, {bw}? Enter the piece's rank", );
            let rank_str = scan_string()
                .replace(" ", "")
                .replace("\n", "");
            println!("Now enter the piece's file." );
            let file_str = scan_string()
                .replace(" ", "")
                .replace("\n", "");
            let rf = match Rankfile::from_strings(rank_str, file_str) {
                Some(rf) => rf,
                None => {println!("Couldn't parse string, try again."); continue;}
            };
            let piece = match board.get_square(rf) {
                Some(p) => p,
                None => {println!("There's nothing there."); continue;}
            };
            if piece.color != whose_turn {
                println!("That's your opponent's piece."); continue;
            }
            break(piece, rf);
        };
        
        let legal_moves = move_validation::get_all_legal_moves(&board, start, piece_moved);
        println!("Great! Here are your legal moves:");
        for mv in legal_moves.iter() {
            let (r, f) = mv.end.to_strings();
            println!("    {r}{f}");
        }

        println!();
        
        let move_executed = loop {
            println!("Which square will you move it to, {bw}? Enter the square's rank", );
            let rank_str = scan_string()
                .replace(" ", "")
                .replace("\n", "");
            println!("Now enter the piece's file." );
            let file_str = scan_string()
                .replace(" ", "")
                .replace("\n", "");
            let rf = match Rankfile::from_strings(rank_str, file_str) {
                Some(rf) => rf,
                None => {println!("Couldn't parse string, try again."); continue;}
            };
            let mut final_move = None;
            for mv in &legal_moves {
                if mv.end == rf {
                    final_move = Some((*mv).clone());
                    break;
                }
            }
            if let Some(final_move) = final_move {
                break(final_move);
            } else {
                println!("That's not a legal move!");
            }
            
        };

        execute_move(&mut board, move_executed);
        
        whose_turn = flip_color(whose_turn);
    }

}