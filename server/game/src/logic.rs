use super::datatypes::{
        board::{GameBoard, 
            position::Rankfile
        },
        piece::{UltimaPiece, UltimaPieceType, PlayerColor},
        moves::MoveData
    };

mod move_validation {
    use std::collections::HashSet;
    use super::*;

    type Direction = (i8, i8);

    const ALL_DIRECTIONS: [Direction; 8] = [(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)];
    const CARDINAL_DIRECTIONS: [Direction; 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    ///Sufficient to determine when a capture will occur.
    struct CaptureData<'board> {
        start: Rankfile,
        end: Rankfile,
        piece: UltimaPiece,
        board: &'board GameBoard
    }

    //For pieces with 'normal' ultima movement:
    //withdrawer, immobilizer, coordinator, pawn
    fn check_standard_piece_moves(
        board: &GameBoard, 
        start: Rankfile, 
        directions: impl Iterator<Item = Direction>,
        piece: UltimaPiece,
        capture_method: impl Fn(CaptureData) -> Vec<Rankfile>
    ) 
        -> HashSet<MoveData> 
    {
        let (r, f) = start.to_signed_coords();
        let mut moves = HashSet::<MoveData>::new();
        for (dr, df) in directions {
            moves.extend((0..8).map_while(|i| {
                //filter out of bounds squares
                Rankfile::from_signed_coords(r + dr * i, f + df * i)
            }).map_while(|rf| {
                //filter by line of sight, return move data.
                if let None = board.get_square(rf) {
                    return None;
                }
                let capture_data = CaptureData {
                    start,
                    end: rf,
                    piece,
                    board,
                };
                Some(
                    MoveData::new(start, rf, capture_method(capture_data))
                )
            }));
        }
        moves
    }

    mod longleaper {
        use super::*;
        pub fn check_longleaper_moves (
            board: &GameBoard, 
            start: Rankfile,
            color: PlayerColor
        ) 
            -> HashSet<MoveData>
        {
            let mut moves = HashSet::<MoveData>::new();
            let (r, f) = start.to_signed_coords();
            for (dr, df) in ALL_DIRECTIONS {
                let mut prev_target_occupied = false;
                let mut dr_mut = dr; let mut df_mut = df;
                let mut captures = vec![];
                while let Some(rf) = Rankfile::from_signed_coords(r + dr_mut, f + df_mut) {
                    match board.get_square(rf) {
                        Some(p) => {
                            if p.color == color {
                                break;
                            } else if !prev_target_occupied {
                                prev_target_occupied = true;
                                //proceed
                            } else {
                                break;
                            }
                        },
                        None => {} //proceed
                    }

                    if prev_target_occupied {
                        captures.push(rf);
                    }

                    moves.insert(MoveData::new(
                        start,
                        rf,
                        captures.clone()
                    ));

                    dr_mut += dr; df_mut += df;
                }
            }

            todo!();
        }
    }
    
    mod pawn {
        use super::*;
        fn pawn_capture_checker<'board>(capture_data: CaptureData<'board>) -> Vec<Rankfile> {
            let CaptureData {
                start,
                end,
                piece,
                board,
            } = capture_data;
            let (r, f) = end.to_signed_coords();
            let mut captures = vec![];
            for (dr, df) in ALL_DIRECTIONS {
                let surrounding_piece =
                match board.get_square_from_coords(r + 2 * dr, f + 2 * df) {
                    None => continue,
                    Some(p) => p
                };
                if surrounding_piece.color == piece.color {
                    if let Some(rf) = Rankfile::from_signed_coords(r, f) {
                        captures.push(rf);
                    }
                }
            }
            captures
        }
    }

    mod coordinator {
        use super::*;
        fn coordinator_capture_checker<'board>(capture_data: CaptureData<'board>) -> Vec<Rankfile> {
            let CaptureData {
                start,
                end,
                piece,
                board,
            } = capture_data;
            let mut captures = vec![];
            for king_pos in board.get_king_locs(piece.color) {
                captures.push(Rankfile{
                    rank: end.rank,
                    file: king_pos.file
                });
                captures.push(Rankfile {
                    rank: king_pos.rank,
                    file: end.file
                });
            }
            captures  
        }
    }

    fn get_all_legal_moves(board: &GameBoard, start: Rankfile, piece: UltimaPiece) -> HashSet<MoveData> {
        unimplemented!();
    }
}

