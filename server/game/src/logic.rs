use super::datatypes::{
        board::{GameBoard, 
            position::Rankfile
        },
        piece::{UltimaPiece, UltimaPieceType, PlayerColor},
        moves::MoveData
    };

pub mod move_validation {

    use std::collections::HashSet;

    use super::*;

    type Direction = (i8, i8);

    const ALL_DIRECTIONS: [Direction; 8] = [(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)];
    const CARDINAL_DIRECTIONS: [Direction; 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    ///Sufficient data to determine when a capture will occur for any piece.
    struct CaptureData<'board> {
        start: Rankfile,
        end: Rankfile,
        piece: UltimaPiece,
        board: &'board GameBoard
    }

    //For pieces with 'normal' ultima movement:
    //withdrawer, immobilizer, coordinator, pawn    
    fn generate_standard_piece_moves(
        board: &GameBoard, 
        start: Rankfile, 
        directions: impl Iterator<Item = &'static Direction>,
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

    
    mod piece_checkers {
        use super::*;
        pub mod pawn {
            use super::*;
            pub fn capture_checker<'board>(capture_data: CaptureData<'board>) -> Vec<Rankfile> {
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

        pub mod immobilizer {
            //no captures can occur.
        }
    
        pub mod coordinator {
            use super::*;
            pub fn capture_checker<'board>(capture_data: CaptureData<'board>) -> Vec<Rankfile> {
                let CaptureData {
                    start: _start,
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

        pub mod longleaper {
            use super::*;
            pub fn generate_moves (
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

        pub mod chameleon {
            use super::*;
            fn generate_moves() {
                unimplemented!();
            }
        }

        pub mod withdrawer {
            use super::*;
            pub fn capture_checker(capture_data: CaptureData) -> Vec<Rankfile> {
                let CaptureData {
                    start,
                    end,
                    board,
                    piece
                } = capture_data;
                let mut captures = vec![];
                let (end_r, end_f) = end.to_signed_coords();
                let mut end_surrounding_squares = HashSet::<Rankfile>::new();
                for (dr, df) in ALL_DIRECTIONS {
                    if let Some(rf) = Rankfile::from_signed_coords(end_r + dr, end_f + df) {
                        end_surrounding_squares.insert(rf);
                    }
                }
                let (start_r, start_f) = start.to_signed_coords();
                for (dr, df) in ALL_DIRECTIONS {
                    if let Some(rf) = Rankfile::from_signed_coords(start_r + dr, start_f + df) {
                        if !end_surrounding_squares.contains(&rf) {
                            //if we started out move adjacent to rf, and ended it not adjacent
                            captures.push(rf);
                        }
                    }
                }
                captures
            }
        }

        pub mod king {
            //for now, ignore checkmate, just play by capture-the-king rules.
            use super::*;
            pub fn generate_moves(start: Rankfile) -> HashSet<MoveData> {
                let mut moves = HashSet::<MoveData>::new();
                let (r, f) = start.to_signed_coords();
                for (dr, df) in ALL_DIRECTIONS {
                    if let Some(rf) = Rankfile::from_signed_coords(r + dr, f + df) {
                        moves.insert(MoveData::new(
                            start,
                            rf,
                            vec![rf]
                        ));
                    }
                }
                moves
            }
            
        }
        
    }
    use piece_checkers::{pawn, king, coordinator, withdrawer, immobilizer, longleaper, chameleon};

    pub fn get_all_legal_moves(board: &GameBoard, start: Rankfile, piece: UltimaPiece) -> HashSet<MoveData> {
        match piece.piece_type {
            UltimaPieceType::Chameleon => {
                todo!();
            },
            UltimaPieceType::Coordinator => {
                generate_standard_piece_moves(
                    board, 
                    start, 
                    ALL_DIRECTIONS.iter(), 
                    piece, 
                    coordinator::capture_checker
                )
            },
            UltimaPieceType::Immobilizer => {
                generate_standard_piece_moves(
                    board, 
                    start, 
                    ALL_DIRECTIONS.iter(), 
                    piece, 
                    |_capture_data| vec![])
            },
            UltimaPieceType::King => {
                king::generate_moves(start)
            },
            UltimaPieceType::Longleaper => {
                longleaper::generate_moves(board, start, piece.color)
            },
            UltimaPieceType::Pawn => {
                generate_standard_piece_moves(
                    board,
                    start, 
                    CARDINAL_DIRECTIONS.iter(), 
                    piece, 
                    pawn::capture_checker
                )
            },
            UltimaPieceType::Withdrawer => {
                generate_standard_piece_moves(
                    board, 
                    start, 
                    ALL_DIRECTIONS.iter(), 
                    piece, 
                    withdrawer::capture_checker
                )
            }     
        }
    }
}