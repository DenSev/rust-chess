use board::Board;
use board::cell::Cell;
use board::piece::*;
use std::collections::HashMap;

mod player;
mod board;
mod game;

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    pub fn test_black_is_opposing_to_white() {
        let black_pawn = Cell::some(Piece {
            piece_kind: PieceKind::Pawn,
            color: Color::Black,
            representation: "black_pawn",
        });

        let white_pawn = Cell::some(Piece {
            piece_kind: PieceKind::Pawn,
            color: Color::White,
            representation: "white_pawn",
        });

        let is_opposing = black_pawn.is_opposing(&white_pawn);

        assert!(is_opposing)
    }

    #[test]
    pub fn test_cell_at() {
        let board = Board::new();

        let expected_white_rook = Cell::some(Piece { piece_kind: PieceKind::Rook, color: Color::White, representation: "♖" });

        let white_rook = board.cell_at(0, 0);

        assert_eq!(white_rook, &expected_white_rook)
    }

    #[test]
    pub fn test_set_cell_at() {
        let mut board = Board::new();

        let expected_piece = Cell::some(Piece { piece_kind: PieceKind::Rook, color: Color::White, representation: "♖" });

        board.set_cell_at(expected_piece, 7, 0);

        let actual_piece = board.cell_at(7, 0);

        assert_eq!(actual_piece, &expected_piece)
    }

    #[test]
    pub fn test() {


        println!("{:?}", PIECE_REPRESENTATIONS.get(&(Color::White, PieceKind::Pawn)).unwrap());
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum TestEnum {
    Foo,
    Bar
}

const PIECES: Vec<(Color, PieceKind)> = vec!((Color::White, PieceKind::Pawn));
const REPRESENTATIONS: Vec<&str> = vec!("white pawn");

pub const PIECE_REPRESENTATIONS: HashMap<(Color, PieceKind), &str> = PIECES.iter()
    .zip(REPRESENTATIONS.iter())
    .collect();

