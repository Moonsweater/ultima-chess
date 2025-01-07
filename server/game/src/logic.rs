use super::datatypes::{
        board::{GameBoard, 
            rankfile
        },
        piece::{UltimaPiece, UltimaPieceType, PlayerColor},
        moves::MoveData
    };
use rankfile::Rankfile;

pub mod move_validation {
    use std::collections::HashSet;

    use super::*;

    ///Sufficient data to determine when a capture will occur for any piece.
    struct CaptureData<'board> {
        board: &'board GameBoard,
        start: Rankfile,
        end: Rankfile,
        piece: UltimaPiece
    }

    //For pieces with 'normal' ultima movement:
    //withdrawer, immobilizer, coordinator, pawn    

    mod piece_checkers {
        use super::*;
        pub mod pawn {
            use super::*;
            pub fn move_generator_iter<'board>
            (board: &'board GameBoard, start: Rankfile, color: PlayerColor) 
            -> impl Iterator<Item = MoveData> + 'board 
            { 
                Rankfile::cardinal_directions().flat_map(move |&dir| {
                    board.los(start, dir).map(move |rf| {
                        let captures = Rankfile::all_directions().filter_map(|&(dr, df)| { 
                            let (r, f) = rf.to_signed_coords();
                            let surrounding = board.get_square_from_coords(r + 2 * dr, f + 2 * df)?;
                            if surrounding.color == color {
                                Some(Rankfile::from(r + dr, f + df)?)
                            } else {None}
                        }).collect();
                        MoveData{start, end: rf, captures}
                    })
                })
            }
        }

        pub mod immobilizer {
            use super::*;

            //In addition to checking for legal moves and captures, 
            //we provide a function to see if some OTHER piece is next to an immobilizer or not.
            //This also checks to see if an immobilizer is immobilized by a chameleon.
           
            pub fn is_immobilized(board: &GameBoard, location: Rankfile, piece: UltimaPiece) -> bool {
                use UltimaPieceType::*;
                let (r, f) = location.to_signed_coords();
                for (dr, df) in Rankfile::all_directions() {
                    let Some(rf) = Rankfile::from(r + dr, f + df) else {continue};
                    let Some(adjacent_piece) = board.get_square(rf) else {continue};
                    if adjacent_piece.piece_type == Immobilizer 
                        && piece.color != adjacent_piece.color 
                    {
                        return true;
                    }
                    if piece.piece_type == Immobilizer
                        && adjacent_piece.piece_type == Chameleon
                        && piece.color != adjacent_piece.color 
                    {
                        return true;
                    }
                }
                false
            }

            pub fn move_generator_iter<'board> 
            (board: &'board GameBoard, start:Rankfile)
            -> impl Iterator<Item = MoveData> + 'board
            {
                Rankfile::all_directions().flat_map(move |&dir| {
                    board.los(start, dir).map(move |rf| {
                        MoveData::new(start, rf, vec![])
                    })
                })
            }
        }
    
        pub mod coordinator {
            use super::*;

            pub fn move_generator_iter<'board>
            (board: &'board GameBoard, start: Rankfile, color: PlayerColor) 
            -> impl Iterator<Item = MoveData> + 'board 
            {
                Rankfile::all_directions().flat_map(move |&dir| {
                    board.los(start, dir).map(move |rf| {
                        let mut captures = vec![];
                        for king in board.get_king_locs(color) {
                            captures.push(Rankfile{rank: rf.rank, file: king.file});
                            captures.push(Rankfile{rank: king.rank, file: rf.file});
                        }
                        MoveData::new(start, rf, captures)
                    })
                })
            }
        }

        pub mod longleaper {
            use super::*;
            pub fn generate_moves<'board> (board: &'board GameBoard, start: Rankfile, color: PlayerColor) -> HashSet<MoveData>
            {
                let mut moves = HashSet::<MoveData>::new();
                let (r, f) = start.to_signed_coords();
                for (dr, df) in Rankfile::all_directions() {
                    let mut leapt_prev_target = false;
                    let mut dr_mut = *dr; let mut df_mut = *df;
                    let mut captures = vec![];
                    while let Some(rf) = Rankfile::from(r + dr_mut, f + df_mut) {
                        if let Some(piece) =  board.get_square(rf) {
                            if piece.color == color {
                                break;
                            } else if leapt_prev_target {
                                break; //can't leap two adjacent enemies
                            } else {
                                leapt_prev_target = true;
                                continue; //can't move onto this square, but maybe the next is good.
                            }
                        } else {
                            leapt_prev_target = false;
                        }
                        if leapt_prev_target {
                            captures.push(rf);
                        }
                        moves.insert(MoveData::new(
                            start,
                            rf,
                            captures.clone()
                        ));
                    }
                }
                moves
            }
        }

        pub mod chameleon {

            //Errata: We declare that a chameleon adjacent to a king can always capture that king, even if the square is defended.
            //This is because, although a chameleon must move like a king, and thus not move into squares that could allow it to be captured the following turn,
            
            //Note: chameleons immobilizing immobilizers is handled inside the `immobilizer` module.

            use super::*;

        }
        pub mod withdrawer {
            use super::*;
            pub fn move_generator_iter<'board> (board: &'board GameBoard, start: Rankfile, color: PlayerColor)
            -> impl Iterator<Item = MoveData> + 'board 
            {
                Rankfile::all_directions().flat_map(move |&dir| {
                    board.los(start, dir).map(move |rf| {
                        (start.surrounding_rankfiles().filter_map(|adj_start| {
                            let piece = board.get_square(adj_start)?;
                            if piece.color != color {Some(rf)} else {None}
                        }).filter(|&adj_start| {
                            for adj_end in rf.surrounding_rankfiles() {
                                if adj_end == adj_start {return true;}
                            }
                            false
                        }).collect(), rf)
                    }).map(move |(captures, end)| {
                        MoveData {start, end, captures}
                    })  
                })
            }
        }

        pub mod king {
            //for now, ignore checkmate, just play by capture-the-king rules.
            use super::*;
            pub fn move_generator_iter(board: &GameBoard, start: Rankfile, color: PlayerColor) -> HashSet<MoveData> {
                let mut moves = HashSet::<MoveData>::new();
                moves.extend(start.surrounding_rankfiles().filter_map(|rf| {
                    let Some(piece) = board.get_square(rf) 
                    else {
                        return Some(MoveData{start, end: rf, captures: vec![]});
                    };
                    if piece.color != color {
                        return Some(MoveData{start, end: rf, captures: vec![rf]});
                    } else {
                        return None;
                    }
                }));
                moves
            }
            
        }
        
    }
    use piece_checkers::{chameleon, coordinator::{self, move_generator_iter}, immobilizer::{self, is_immobilized}, king, longleaper, pawn, withdrawer};

    pub fn get_all_legal_moves(board: &GameBoard, start: Rankfile, piece: UltimaPiece) -> HashSet<MoveData> {
        use UltimaPieceType::*;
        if is_immobilized(board, start, piece) {return HashSet::new()}
        let color = piece.color;
        match piece.piece_type {
            Chameleon => {
                todo!();
            },
            Coordinator => {
                coordinator::move_generator_iter(board, start, color).collect()
            },
            Immobilizer => {
                immobilizer::move_generator_iter(board, start).collect()
            },
            King => {
                king::move_generator_iter(board, start, color)
            },
            Longleaper => {
                longleaper::generate_moves(board, start, color)
            },
            Pawn => {
                pawn::move_generator_iter(board, start, color).collect()
            },
            Withdrawer => {
                withdrawer::move_generator_iter(board, start, color).collect()
            }     
        }
    }
}

pub mod board_changes {
    use super::*;
    fn execute_move(board: &mut GameBoard, move_to_execute: MoveData) {
        let MoveData {
            start,
            end,
            captures
        } = move_to_execute;
        for square in captures {
            board.set_square(square, None);
        }
        board.set_square(end, board.get_square(start));
        board.set_square(start, None);
    }
}