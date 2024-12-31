mod datatypes;
mod logic;

pub use logic::move_validation;
pub use datatypes::{
    board::{GameBoard, 
        position::Rankfile
    },
    piece::{UltimaPiece, UltimaPieceType, PlayerColor},
    moves::MoveData
};

pub use logic::*;