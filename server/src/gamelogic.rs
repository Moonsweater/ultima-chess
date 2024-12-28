use std::collections::HashSet;

#[derive(PartialEq, Eq)]
pub enum UltimaPieceType {
    Pawn,
    Immobilizer,
    Coordinator,
    Longleaper,
    Chameleon,
    Withdrawer,
    King
}

#[derive(PartialEq, Eq)]
pub enum PlayerColor {
    Black,
    White
}

#[derive(PartialEq, Eq)]
struct UltimaPiece {
    piece_type: UltimaPieceType,
    color: PlayerColor
}

#[derive(PartialEq, Eq, Hash)]
enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8
}

#[derive(PartialEq, Eq, Hash)]
//chess file, not system file
enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

#[derive(PartialEq, Eq, Hash)]
struct Rankfile {
    rank: Rank,
    file: File
}

enum MoveError {
    PieceMismatch(PieceMismatchError),
    IllegalMove
    //could specify why a move is illegal...?
}

struct PieceMismatchError {
    //For now, we act like there is only one way to have such an error.
    //probably should be a struct? But maybe could be an enum if we do manual 'inheritance'.
}

impl Rankfile {

    pub fn as_coords(&self) -> (usize, usize) {
        (match &(self.rank) {
            Rank::R1 => 0,
            Rank::R2 => 1,
            Rank::R3 => 2,
            Rank::R4 => 3,
            Rank::R5 => 4,
            Rank::R6 => 5,
            Rank::R7 => 6,
            Rank::R8 => 7
        },
        match &(self.file) {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7
        })
    }

    fn rank_from_coord(rank_int: usize) -> Result<Rank, ()> {
        match rank_int {
            0 => Ok(Rank::R1),
            1 => Ok(Rank::R2),
            2 => Ok(Rank::R3),
            3 => Ok(Rank::R4),
            4 => Ok(Rank::R5),
            5 => Ok(Rank::R6),
            6 => Ok(Rank::R7),
            7 => Ok(Rank::R8),
            _ => Err(())
        }
    }

    fn file_from_coord(file_int: usize) -> Result<File, ()> {
        match file_int {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(())
        }
    }

    pub fn from_coords(rank_int: usize, file_int: usize) -> Self {
        //danger: unwraps.
        Self {
            rank: Self::rank_from_coord(rank_int).unwrap(),
            file: Self::file_from_coord(file_int).unwrap()
        }
    }

    pub fn from_string() -> Self {
        todo!();
    }

    //probably useless, since we need to test for obstruction anyway. :p

    fn is_in_cardinal_line(a: &Self, b: &Self) -> bool {
        return a.rank == b.rank || a.file == b.file
    }

    fn is_in_diagonal_line(a: &Self, b: &Self) -> bool {

        let a_coords = a.as_coords();
        let b_coords = b.as_coords();

        if (a_coords.0 + a_coords.1 == b_coords.0 + b_coords.1) {
            //top left to bottom right diagonal
            return true;
        }

        if (a_coords.0 as i8 - a_coords.1 as i8 == b_coords.0 as i8 - b_coords.1 as i8) {
            //bottom left to top tight diagonal
            //casting needed for possibility of negative values
            return true;
        }

        return false;

    }

}

struct GameBoard([[Option<UltimaPiece>; 8]; 8]);

//Rows are ranks: indicated in std notation via numerals, indexed at 1.
//Columns are files: indicated in std notation via letters, indexed at a.

//GameBoard stores positions as board[i][j] <=> rank == (i+1), file == alphabet(j+1).

impl GameBoard {

    pub fn view_square<'boardlife> (&'boardlife self, pos: Rankfile) -> &'boardlife Option<UltimaPiece> {
        let (r, f) = pos.as_coords();
        &(self.0[r][f])
    }

    pub fn new_empty() -> Self {

        //silly hack to bypass array init restrictions on non-copy types
        const EMPTY_PIECE: Option<UltimaPiece> = None;
        const EMPTY_RANK: [Option<UltimaPiece>; 8] = [EMPTY_PIECE; 8];
        //possible that the consts should be at a higher scope somewhere. Revisit this

        GameBoard([EMPTY_RANK; 8])
    }

    pub fn new_in_start_position() -> Self {
        let mut out = Self::new_empty().0;
        for file in 0..8 {
            out[1][file] = Some(UltimaPiece {
                piece_type: UltimaPieceType::Pawn,
                color: PlayerColor::White
            });
            out[6][file] = Some(UltimaPiece {
                piece_type: UltimaPieceType::Pawn,
                color: PlayerColor::Black
            });
        }

        out[0][0] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Immobilizer,
            color: PlayerColor::White
        });
        out[7][7] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Immobilizer,
            color: PlayerColor::Black
        });

        //===============

        out[0][7] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Coordinator,
            color: PlayerColor::White
        });
        out[7][0] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Coordinator,
            color: PlayerColor::Black
        });

        //===============

        out[0][1] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Longleaper,
            color: PlayerColor::White
        });
        out[0][6] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Longleaper,
            color: PlayerColor::White
        });

        out[7][1] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Longleaper,
            color: PlayerColor::Black
        });
        out[7][6] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Longleaper,
            color: PlayerColor::Black
        });


        //===============

        out[0][2] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Chameleon,
            color: PlayerColor::White
        });
        out[0][5] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Chameleon,
            color: PlayerColor::White
        });

        out[7][2] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Chameleon,
            color: PlayerColor::Black
        });
        out[7][5] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Chameleon,
            color: PlayerColor::Black
        });

        //===============

        out[0][3] = Some(UltimaPiece {
            piece_type: UltimaPieceType::King,
            color: PlayerColor::White
        });
        out[7][4] = Some(UltimaPiece {
            piece_type: UltimaPieceType::King,
            color: PlayerColor::Black
        });

        //===============

        out[0][4] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Withdrawer,
            color: PlayerColor::White
        });
        out[7][3] = Some(UltimaPiece {
            piece_type: UltimaPieceType::Withdrawer,
            color: PlayerColor::Black
        });

        GameBoard(out)

    }

    // Validates whether a proposed piece matches the piece at the given rankfile of a board.
    // If valid, returns an empty Ok. Else, returns an Err containing a descriptive error enum.
    fn board_matches(&self, piece: UltimaPiece, pos: Rankfile) -> Result<(), PieceMismatchError> {
        let wrapped_piece = &Some(piece);
        let wrapped_board_piece = self.view_square(pos);
        match (wrapped_piece, wrapped_board_piece) {
            (Some(p), Some(b)) => {
                if p.color != b.color {
                    return Err(PieceMismatchError{});
                }
                if p.piece_type != b.piece_type {
                    return Err(PieceMismatchError{});
                }
                return Ok(());
            }
            _ => {return Err(PieceMismatchError{});}
        }
    }

    /// Computes the set of all legal targets from `start`,
    /// assuming that a piece of the type and color matching `piece` is located at `start`.
    pub fn generate_all_legal_targets(&self, piece: &UltimaPiece, start: &Rankfile) -> HashSet<Rankfile> {
        let (start_rank, start_file) = start.as_coords();
        let mut legal_targets: HashSet<Rankfile> = HashSet::new();

        let check_standard_piece_cardinals = |legal_targets: &mut HashSet<Rankfile>| {
            //up from start:
            for current_rank in (start_rank+1)..8 {
                let current_square = &(self.0[current_rank][start_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(current_rank, start_file));}
                }
            }
            //down from start:
            for current_rank in (0..start_rank).rev() {
                let current_square = &(self.0[current_rank][start_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(current_rank, start_file));}
                }
            }
            //left from start:
            for current_file in (0..start_file).rev() {
                let current_square = &(self.0[start_rank][current_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(start_rank, current_file));}
                }
            }
            //right from start:
            for current_file in ((start_file+1)..8).rev() {
                let current_square = &(self.0[start_rank][current_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(start_rank, current_file));}
                }
            }
        };

        let check_standard_piece_diagonals = |legal_targets: &mut HashSet<Rankfile>| {
            
            let mut current_rank;
            let mut current_file;

            //towards top-right:
            current_rank = start_rank + 1;
            current_file = start_file + 1;

            while current_rank < 8 && current_file < 8 {
                let current_square = &(self.0[current_rank][current_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(current_rank, start_file));}
                }
                current_rank += 1;
                current_file += 1;
            }

            //towards top-left:
            current_rank = start_rank + 1;
            current_file = start_file - 1;

            while current_rank < 8 && current_file <= 0 {
                let current_square = &(self.0[current_rank][current_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(current_rank, start_file));}
                }
                current_rank += 1;
                current_file -= 1;
            }

            //towards bottom-left:         
            current_rank = start_rank - 1;
            current_file = start_file - 1;

            while current_rank >= 0 && current_file >= 0{
                let current_square = &(self.0[current_rank][current_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(current_rank, start_file));}
                }
                current_rank -= 1;
                current_file -= 1;
            }

            //towards bottom-right:        
            current_rank = start_rank - 1;
            current_file = start_file + 1;

            while current_rank  >= 0 && current_file < 8 {
                let current_square = &(self.0[current_rank][current_file]);
                match current_square {
                    Some(_piece) => {break;}
                    None => {legal_targets.insert(Rankfile::from_coords(current_rank, start_file));}
                }
                current_rank -= 1;
                current_file += 1;
            }
        };

        match piece.piece_type {
            UltimaPieceType::Pawn => {
                check_standard_piece_cardinals(&mut legal_targets);
            },
            UltimaPieceType::King => {
                
            },
            UltimaPieceType::Withdrawer => {
                check_standard_piece_cardinals(&mut legal_targets);
                check_standard_piece_diagonals(&mut legal_targets);
            },
            UltimaPieceType::Chameleon => {},
            UltimaPieceType::Longleaper => {},
            UltimaPieceType::Coordinator => {
                check_standard_piece_cardinals(&mut legal_targets);
                check_standard_piece_diagonals(&mut legal_targets);
            },
            UltimaPieceType::Immobilizer => {
                check_standard_piece_cardinals(&mut legal_targets);
                check_standard_piece_diagonals(&mut legal_targets);
            }
        }

        todo!();
    }

}