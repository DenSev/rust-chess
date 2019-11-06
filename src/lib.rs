use std::collections::HashMap;
use std::option::Option;
use std::fs::read;

const BOARD_SIZE: usize = 8;

pub enum Cell {
    None,
    //Some(Piece),
}


#[derive(Debug, PartialEq, Eq)]
pub struct Piece{
    pub piece_kind: PieceKind,
    pub color: Color,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
pub enum PieceKind {
    Queen,
    King,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

pub trait Movement {
    fn get_available_movement(&self) -> Vec<Pos>;
}

pub struct Pawn {
}

pub trait Presentable {
    fn get_representation(&self) -> &'static str;
}

impl Presentable for Piece {
    fn get_representation(&self) -> &'static str {
        match &self.color {
            Color::Black => {
                match &self.piece_kind {
                    PieceKind::Bishop=>"♝",
                    PieceKind::King=>"♚",
                    PieceKind::Knight=>"♞",
                    PieceKind::Pawn=>"♟",
                    PieceKind::Queen=> "♛",
                    PieceKind::Rook=>"♜",
                }
            },
            Color::White => {
                match &self.piece_kind {
                    PieceKind::Bishop=>"♗",
                    PieceKind::King=>"♔",
                    PieceKind::Knight=>"♘",
                    PieceKind::Pawn=>"♙",
                    PieceKind::Queen=>"♕",
                    PieceKind::Rook=>"♖",
                }
            }
        }
    }
}

impl Movement for Pawn {
    fn get_available_movement(&self) -> Vec<Pos> {
        println!("pawn moves");
        return Vec::new();
    }
}

#[derive(Eq, PartialEq)]
pub struct Pos {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_black_is_opposing_to_white() {
        let black_pawn = Piece {
            piece_kind: PieceKind::Pawn,
            color: Color::Black,
        };

        let white_pawn = Piece {
            piece_kind: PieceKind::Pawn,
            color: Color::White,
        };

        println!("this is test {} ",  black_pawn.get_representation());
        assert!(is_opposing)
    }

}