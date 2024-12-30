pub mod piece {
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub struct UltimaPiece {
        pub piece_type: UltimaPieceType,
        pub color: PlayerColor
    }
    
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum UltimaPieceType {
        Pawn,
        Immobilizer,
        Coordinator,
        Longleaper,
        Chameleon,
        Withdrawer,
        King
    }
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum PlayerColor {
        Black,
        White
    }
}
use piece::{UltimaPiece, UltimaPieceType, PlayerColor};

pub mod board {
    use super::piece::{PlayerColor, UltimaPiece};
    pub type Square = Option<UltimaPiece>;
    pub mod position {
        #[derive(PartialEq, Eq, Hash, Clone, Copy)]
        pub enum Rank {
            R1,
            R2,
            R3,
            R4,
            R5,
            R6,
            R7,
            R8
        } 
        impl Rank {
            pub fn as_index(&self) -> i8 {
                match self {
                    Rank::R1 => 0, Rank::R2 => 1, 
                    Rank::R3 => 2, Rank::R4 => 3,
                    Rank::R5 => 4, Rank::R6 => 5, 
                    Rank::R7 => 6, Rank::R8 => 7
                }
            }
            pub fn new_if_exists(r: i8) -> Option<Self> {
                let out = match r {
                    0 => Rank::R1, 1 => Rank::R2, 
                    2 => Rank::R3, 3 => Rank::R4, 
                    4 => Rank::R5, 5 => Rank::R6, 
                    6 => Rank::R7, 7 => Rank::R8,
                    _ => {return None;}
                };
                Some(out)
            }
        }
        #[derive(PartialEq, Eq, Hash, Clone, Copy)]
        //chess file, not system file
        pub enum File {
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H
        }  
        impl File {
            pub fn as_index(&self) -> i8 {
                match self {
                    File::A => 0, File::B => 1,
                    File::C => 2, File::D => 3,
                    File::E => 4, File::F => 5,
                    File::G => 6, File::H => 7
                }
            }
            pub fn new_if_exists(r: i8) -> Option<Self> {
                let out = match r {
                    0 => File::A, 1 => File::B,
                    2 => File::C, 3 => File::D, 
                    4 => File::E, 5 => File::F,
                    6 => File::G, 7 => File::H,
                    _ => {return None;}
                };
                Some(out)
            }
        }
        #[derive(PartialEq, Eq, Hash, Clone, Copy)]
        pub struct Rankfile {
            pub rank: Rank,
            pub file: File
        }
        impl Rankfile {
            pub fn from_signed_coords(r: i8, f:i8) -> Option<Rankfile> {
                Some(Self {
                    rank: Rank::new_if_exists(r)?,
                    file: File::new_if_exists(f)?
                })
            }
            pub fn to_signed_coords(&self) -> (i8, i8) {
                (self.rank.as_index(), self.file.as_index())
            }
            pub fn to_unsigned_coords(&self) -> (usize, usize) {
                (self.rank.as_index() as usize, self.file.as_index() as usize)
            }

        }
    }
    use position::Rankfile;

    pub struct GameBoard {
        board:[[Square; 8]; 8],
        black_king_locs: Vec<Rankfile>,
        white_king_locs: Vec<Rankfile> //fully general, allows for silly boards with multiple kings.
    }
    mod board_init_consts {
        use super::{UltimaPiece, Square, Rankfile};
        use super::position::{Rank, File};
        use super::super::piece::{UltimaPieceType, PlayerColor};

        const EMPTY_SQUARE: Square = None;
        pub const EMPTY_RANK: [Square; 8] = [EMPTY_SQUARE; 8];

        const W_PAWN: UltimaPiece = UltimaPiece {
            color: PlayerColor::White,
            piece_type: UltimaPieceType::Pawn
        };
        const B_PAWN: UltimaPiece = UltimaPiece {
            color: PlayerColor::Black,
            piece_type: UltimaPieceType::Pawn
        };

        const W_PAWN_RANK: [Square; 8] = [Some(W_PAWN); 8];
        const B_PAWN_RANK: [Square; 8] = [Some(B_PAWN); 8];

        const W_BACK_RANK: [Square; 8] = [
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::Immobilizer
            }),
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::Longleaper
            }),
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::Chameleon
            }),
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::King
            }),
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::Withdrawer
            }),
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::Chameleon
            }),
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::Longleaper
            }),
            Some(UltimaPiece {
                color: PlayerColor::White,
                piece_type: UltimaPieceType::Coordinator
            })
        ];

        const B_BACK_RANK: [Square; 8] = [
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::Immobilizer
            }),
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::Longleaper
            }),
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::Chameleon
            }),
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::King
            }),
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::Withdrawer
            }),
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::Chameleon
            }),
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::Longleaper
            }),
            Some(UltimaPiece {
                color: PlayerColor::Black,
                piece_type: UltimaPieceType::Coordinator
            })
        ];

        pub const START_BOARD: [[Square; 8]; 8] = [
            B_BACK_RANK,
            B_PAWN_RANK,
            EMPTY_RANK,
            EMPTY_RANK,
            EMPTY_RANK,
            EMPTY_RANK,
            W_PAWN_RANK,
            W_BACK_RANK
        ];

        pub const W_KING_LOC: Rankfile = Rankfile{rank: Rank::R1, file: File::D};
        pub const B_KING_LOC: Rankfile = Rankfile{rank: Rank::R8, file: File::E};

    }
    
    impl GameBoard {
        pub fn get_square(&self, rf: Rankfile) -> Square {
            let (r, f) = rf.to_unsigned_coords();
            self.board[r][f]
        }
        pub fn get_square_from_coords(&self, r: i8, f: i8) -> Square {
            let rf = match Rankfile::from_signed_coords(r, f) {
                None => return None,
                Some(rf) => rf
            };
            return self.get_square(rf);
        }
        pub fn new_empty() -> Self {
            GameBoard {
                board: [board_init_consts::EMPTY_RANK; 8],
                black_king_locs: vec![],
                white_king_locs: vec![]
            }
        }
        pub fn new_in_start_position() -> Self {
            Self{
                board: board_init_consts::START_BOARD,
                black_king_locs: vec![board_init_consts::B_KING_LOC],
                white_king_locs: vec![board_init_consts::W_KING_LOC]

            }
        }
        pub fn get_king_locs(&self, color: PlayerColor) -> &Vec<Rankfile> {
            match color {
                PlayerColor::White => &self.white_king_locs,
                PlayerColor::Black => &self.black_king_locs
            }
        }
    }
}
use board::GameBoard;
use board::position::Rankfile;

pub mod moves {
    use super::Rankfile;

    #[derive(PartialEq, Eq, Hash)]
    pub struct MoveData {
        start: Rankfile,
        end: Rankfile,
        captures: Vec<Rankfile>
    }

    impl MoveData {
        pub fn new(start: Rankfile, end: Rankfile, captures: Vec<Rankfile>) -> Self {
            MoveData {
                start, end, captures
            }
        }
    }
}
