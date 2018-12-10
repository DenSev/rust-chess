
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_kind: PieceKind,
    pub color: Color,
    pub representation: &'static str,
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

