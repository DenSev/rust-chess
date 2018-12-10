use board::piece::Piece;
use board::piece::Color;
use board::piece::PieceKind;


#[derive(Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    None,
    Some(Piece),
}

impl Cell {
    pub fn none() -> Cell {
        return Cell::None;
    }

    pub fn some(piece: Piece) -> Cell {
        return Cell::Some(piece);
    }

    pub fn is_empty(&self) -> bool {
        return match &self {
            Cell::None => false,
            Cell::Some(_) => true
        };
    }

    pub fn is_not_empty(&self) -> bool {
        return match &self {
            Cell::None => true,
            Cell::Some(_) => false
        };
    }

    pub fn get_representation(&self) -> &str {
        return match &self {
            Cell::None => " ",
            Cell::Some(piece) => &piece.representation
        };
    }

    pub fn get_color(&self) -> Option<&Color> {
        return match &self {
            Cell::None => None,
            Cell::Some(piece) => Some(&piece.color)
        };
    }

    pub fn get_piece(&self) -> Option<&Piece> {
        return match &self {
            Cell::None => None,
            Cell::Some(piece) => Some(&piece)
        };
    }

    pub fn get_piece_kind(&self) -> Option<&PieceKind> {
        return match &self {
            Cell::None => None,
            Cell::Some(piece) => Some(&piece.piece_kind)
        };
    }


    pub fn is_opposing(&self, cell: &Cell) -> bool {
        return match &self {
            Cell::None => false,
            Cell::Some(piece) => piece.color.ne(&cell.get_color().unwrap())
        };
    }

    pub fn is_not_opposing(&self, cell: &Cell) -> bool {
        return match &self {
            Cell::None => true,
            Cell::Some(piece) => piece.color.eq(&cell.get_color().unwrap())
        };
    }
}
