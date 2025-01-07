mod datatypes;
mod logic;


pub use datatypes::{
    board::{GameBoard, 
        rankfile::Rankfile
    },
    piece::{UltimaPiece, UltimaPieceType, PlayerColor},
    moves::MoveData
};

pub use logic::*;