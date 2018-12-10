use board::Board;
use board::piece::Color;
use super::Player;

pub trait Factory<T: Player> {
    fn get_player(&self, board: &Board, color: Color) -> T;
}

pub struct Container<'a> {
    player: &'a (Player + 'a)
}

impl <'a> Container<'a> {
    pub fn play(&self){
        self.player.make_a_move_and_check()
    }
}

pub fn get_player<T>(player: &T) -> Container where T: Player{
    return Container { player};
}