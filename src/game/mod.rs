use board::piece::Color;
use super::board::*;
use super::player::*;

pub struct Game {
    board: Board,
    checkmate: bool,
}

impl Game {}

pub fn play() {
    let board = Board::new();
    board.print();
    let mut checkmate = false;
    //let white_player = CommandLinePlayer { board, color: Color::White };
    let black_player = DoNothingPlayer { board, color: Color::Black };

    let player_container = factory::get_player(&black_player);
    player_container.play();


    while !checkmate {

    }
}