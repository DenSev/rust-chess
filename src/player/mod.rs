use board::Board;
use board::piece::Color;

pub mod factory;

pub trait Player {
    fn check(&self);

    fn make_a_move(&self);

    fn make_a_move_and_check(&self) {
        self.make_a_move();
        self.check();
    }
}

pub struct DoNothingPlayer {
    pub board: Board,
    pub color: Color,
}

impl Player for DoNothingPlayer {
    fn check(&self) {
        let checked_color = self.board.check_board();
        if checked_color.is_none() {
            println!("{:?} king has been checked!", checked_color.unwrap());
        }
    }

    fn make_a_move(&self) {
        println!("{:?} player does nothing!", self.color)
    }
}

pub struct CommandLinePlayer {
    pub board: Board,
    pub color: Color,
}

impl Player for CommandLinePlayer {
    fn check(&self) {
        unimplemented!()
    }

    fn make_a_move(&self) {
        unimplemented!()
    }
}