extern crate rust_chess;

use rust_chess::Cell;
use rust_chess::Color;
use rust_chess::Piece;
use rust_chess::Board;
use rust_chess::PieceKind;

fn main() {
    let mut board = Board::new();

    let expected_piece = Cell::some(Piece { piece_kind: PieceKind::Rook, color: Color::White, representation: "♖" });

    board.set_cell_at(expected_piece, 7, 0);

    let actual_piece = board.cell_at(7, 0);

    println!("{:?}", expected_piece)

}
