use board::cell::Cell;
use board::cell::Pos;
use board::piece::Color;
use board::piece::Piece;
use board::piece::PieceKind;
use std::collections::HashMap;

pub mod cell;
pub mod piece;

const BOARD_SIZE: usize = 8;

#[derive(Debug)]
pub struct Board {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    pub fn new() -> Board {
        let white_queen = "♕";
        let white_king = "♔";
        let white_bishop = "♗";
        let white_knight = "♘";
        let white_rook = "♖";
        let white_pawn = "♙";

        let black_queen = "♛";
        let black_king = "♚";
        let black_bishop = "♝";
        let black_knight = "♞";
        let black_rook = "♜";
        let black_pawn = "♟";

        let mut board = Board { cells: [[Cell::None; BOARD_SIZE]; BOARD_SIZE] };
        //rooks
        board.set_cell_at(Cell::some(Piece {
            piece_kind: PieceKind::Rook,
            color: Color::White,
            representation: white_rook,
        }), 0, 0);
        board.cells[0][7] = Cell::some(Piece {
            piece_kind: PieceKind::Rook,
            color: Color::White,
            representation: white_rook,
        });
        board.cells[7][0] = Cell::some(Piece {
            piece_kind: PieceKind::Rook,
            color: Color::Black,
            representation: black_rook,
        });
        board.cells[7][7] = Cell::some(Piece {
            piece_kind: PieceKind::Rook,
            color: Color::Black,
            representation: black_rook,
        });
        //knights
        board.cells[0][1] = Cell::some(Piece {
            piece_kind: PieceKind::Knight,
            color: Color::White,
            representation: white_knight,
        });
        board.cells[0][6] = Cell::some(Piece {
            piece_kind: PieceKind::Knight,
            color: Color::White,
            representation: white_knight,
        });
        board.cells[7][1] = Cell::some(Piece {
            piece_kind: PieceKind::Knight,
            color: Color::Black,
            representation: black_knight,
        });
        board.cells[7][6] = Cell::some(Piece {
            piece_kind: PieceKind::Knight,
            color: Color::Black,
            representation: black_knight,
        });
        //bishops
        board.cells[0][2] = Cell::some(Piece {
            piece_kind: PieceKind::Bishop,
            color: Color::White,
            representation: white_bishop,
        });
        board.cells[0][5] = Cell::some(Piece {
            piece_kind: PieceKind::Bishop,
            color: Color::White,
            representation: white_bishop,
        });
        board.cells[7][2] = Cell::some(Piece {
            piece_kind: PieceKind::Bishop,
            color: Color::Black,
            representation: black_bishop,
        });
        board.cells[7][5] = Cell::some(Piece {
            piece_kind: PieceKind::Bishop,
            color: Color::Black,
            representation: black_bishop,
        });
        //queens
        board.cells[0][3] = Cell::some(Piece {
            piece_kind: PieceKind::Queen,
            color: Color::White,
            representation: white_queen,
        });
        board.cells[7][3] = Cell::some(Piece {
            piece_kind: PieceKind::Queen,
            color: Color::Black,
            representation: black_queen,
        });
        //kings
        board.cells[0][4] = Cell::some(Piece {
            piece_kind: PieceKind::King,
            color: Color::White,
            representation: white_king,
        });
        board.cells[7][4] = Cell::some(Piece {
            piece_kind: PieceKind::King,
            color: Color::Black,
            representation: black_king,
        });

        for i in 0..BOARD_SIZE {
            board.cells[1][i] = Cell::some(Piece {
                piece_kind: PieceKind::Pawn,
                color: Color::White,
                representation: white_pawn,
            });
            board.cells[6][i] = Cell::some(Piece {
                piece_kind: PieceKind::Pawn,
                color: Color::Black,
                representation: black_pawn,
            });
        }

        return board;
    }

    pub fn print(&self) {
        for i in (0..BOARD_SIZE).rev() {
            print!("[{}]\t", i);
            for j in 0..BOARD_SIZE {
                print!("[{}]\t", &self.cells[i][j].get_representation());
            }
            println!()
        }
        println!("[X]\t[0]\t[1]\t[2]\t[3]\t[4]\t[5]\t[6]\t[7]")
    }

    pub fn set_cell_at(&mut self, cell: Cell, x: usize, y: usize) {
        self.cells[y][x] = cell;
    }

    pub fn cell_at(&self, x: usize, y: usize) -> &Cell {
        return &self.cells[y][x];
    }

    pub fn get_cells_of_color(&self, color: Color) -> HashMap<Pos, &Cell> {
        let mut pieces: HashMap<Pos, &Cell> = HashMap::new();

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let cell_at = self.cell_at(i, j);

                if color.eq(cell_at.get_color().unwrap()) {
                    pieces.insert(Pos { x: j, y: i }, cell_at);
                }
            }
        }
        return pieces;
    }

    pub fn get_pos_of_piece(&self, piece: PieceKind, color: Color) -> Option<Pos> {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let cell_at = self.cell_at(i, j);
                if color.eq(cell_at.get_color().unwrap())
                    && piece.eq(cell_at.get_piece_kind().unwrap()) {
                    return Some(Pos { x: j, y: i });
                }
            }
        }
        return None;
    }

    pub fn check_for_pieces(&self, pieces: HashMap<Pos, &Cell>, opposing_king_pos: Pos) -> bool {
        for (pos, piece) in pieces.iter() {
            let positions = Vec::new();//Game.INSTANCE.getPieceMovement(piece).getAvailableMovePositions(pieceAtPosition.getKey());
            if positions.contains(&opposing_king_pos) {
                return true;
            }
        }
        return false;
        /* private boolean checkForPieces(Map<Position, Cell> pieces, Position opposingKingPosition) {
        for (Map.Entry<Position, Cell> pieceAtPosition : pieces.entrySet()) {
            Piece piece = pieceAtPosition.getValue().getPiece();
            List<Position> positions = Game.INSTANCE.getPieceMovement(piece)
                .getAvailableMovePositions(pieceAtPosition.getKey());

            if (positions.contains(opposingKingPosition)) {
                return true;
            }
        }
        return false;
        } */
    }

    pub fn check_board(&self) -> Option<Color> {
        let white_pieces = self.get_cells_of_color(Color::White);
        let black_king = self.get_pos_of_piece(PieceKind::King, Color::Black);
        let black_king_checked = self.check_for_pieces(white_pieces, black_king.unwrap());

        return None;
        /*
    public Color checkBoard() {
        final Map<Position, Cell> whitePieces = getCellsOfColor(WHITE);
        final Position blackKingPosition = getPositionOfPiece(KING, BLACK);
        final boolean blackKingChecked = checkForPieces(whitePieces, blackKingPosition);

        if (blackKingChecked) {
            return Color.BLACK;
        }

        final Map<Position, Cell> blackPieces = getCellsOfColor(BLACK);
        final Position whiteKingPosition = getPositionOfPiece(KING, WHITE);
        final boolean whiteKingChecked = checkForPieces(blackPieces, whiteKingPosition);

        if (whiteKingChecked) {
            return Color.WHITE;
        }

        return null;
    }*/
    }
}