extern crate rust_chess;

use rust_chess::Cell;
use rust_chess::Color;
use rust_chess::Piece;

fn main() {
    let cell = Cell::some(Piece::KING(Color::BLACK, String::from("king")));
    let black_king = cell.get_representation();

    let piece = Piece::KING(Color::BLACK, String::from("king"));
    if let Piece::KING(_color, repr) = piece {
        repr;
    };


    println!("Hello, world! {} ", black_king);
}
