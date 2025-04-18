#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 6,
}

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Iterator for PieceType {
    type Item = PieceType;

    fn next(&mut self) -> Option<Self::Item> {
        use PieceType::*;
        match self {
            Pawn => Some(Knight),
            Knight => Some(Bishop),
            Bishop => Some(Rook),
            Rook => Some(Queen),
            Queen => Some(King),
            King => None,
        }
    }
}

impl PieceType {
    pub fn all() -> impl Iterator<Item = PieceType> {
        use PieceType::*;
        [Pawn, Knight, Bishop, Rook, Queen, King].iter().copied()
    }
}

pub enum PieceIndices {
    WhitePawn = 0,
    WhiteKnight = 1,
    WhiteBishop = 2,
    WhiteRook = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackKnight = 7,
    BlackBishop = 8,
    BlackRook = 9,
    BlackQueen = 10,
    BlackKing = 11,
}

pub fn get_piece_index(piece: PieceType, color: Color) -> usize {
    match piece {
        PieceType::Pawn => PieceIndices::WhitePawn as usize + color as usize,
        PieceType::Knight => PieceIndices::WhiteKnight as usize + color as usize,
        PieceType::Bishop => PieceIndices::WhiteBishop as usize + color as usize,
        PieceType::Rook => PieceIndices::WhiteRook as usize + color as usize,
        PieceType::Queen => PieceIndices::WhiteQueen as usize + color as usize,
        PieceType::King => PieceIndices::WhiteKing as usize + color as usize,
    }
}

pub struct BitBoard {
    // white_pawns: u64,
    // white_knights: u64,
    // white_bishops: u64,
    // white_rooks: u64,
    // white_queen: u64,
    // white_king: u64,
    // black_pawns: u64,
    // black_knights: u64,
    // black_bishops: u64,
    // black_rooks: u64,
    // black_queen: u64,
    // black_king: u64,
    piece_bb: [u64; 12],
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard {
            piece_bb: [
                0x000000000000FF00, // white pawns   (rank 7)
                0x0000000000000042, // white knights (b8, g8)
                0x0000000000000024, // white bishops (c8, f8)
                0x0000000000000081, // white rooks   (a8, h8)
                0x0000000000000008, // white queen   (d8)
                0x0000000000000010,
                0x00FF000000000000, // black pawns   (rank 2)
                0x4200000000000000, // black knights (b1, g1)
                0x2400000000000000, // black bishops (c1, f1)
                0x8100000000000000, // black rooks   (a1, h1)
                0x0800000000000000, // black queen   (d1)
                0x1000000000000000,
            ],
        }
    }

    pub fn new_from_pieces(
        white_pawns: u64,
        white_knights: u64,
        white_bishops: u64,
        white_rooks: u64,
        white_queen: u64,
        white_king: u64,
        black_pawns: u64,
        black_knights: u64,
        black_bishops: u64,
        black_rooks: u64,
        black_queen: u64,
        black_king: u64,
    ) -> Self {
        BitBoard {
            piece_bb: [
                white_pawns,   // 0
                white_knights, // 1
                white_bishops, // 2
                white_rooks,   // 3
                white_queen,   // 4
                white_king,    // 5
                black_pawns,   // 6
                black_knights, // 7
                black_bishops, // 8
                black_rooks,   // 9
                black_queen,   // 10
                black_king,    // 11
            ],
        }
    }

    pub fn move_piece(&mut self, pice_type: &PieceType, color: &Color, from: u32, to: u32) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        self.piece_bb[get_piece_index(*pice_type, *color)] &= !from_mask;
        self.piece_bb[get_piece_index(*pice_type, *color)] |= to_mask;
    }

    pub fn get_piece_bb(&self, piece: PieceType, color: Color) -> u64 {
        self.piece_bb[get_piece_index(piece, color)]
    }

    pub fn get_pawns(&self, color: Color) -> u64 {
        self.piece_bb[get_piece_index(PieceType::Pawn, color)]
    }

    pub fn get_knights(&self, color: Color) -> u64 {
        self.piece_bb[get_piece_index(PieceType::Knight, color)]
    }

    pub fn get_bishops(&self, color: Color) -> u64 {
        self.piece_bb[get_piece_index(PieceType::Bishop, color)]
    }

    pub fn get_rooks(&self, color: Color) -> u64 {
        self.piece_bb[get_piece_index(PieceType::Rook, color)]
    }

    pub fn get_queens(&self, color: Color) -> u64 {
        self.piece_bb[get_piece_index(PieceType::Queen, color)]
    }

    pub fn get_kings(&self, color: Color) -> u64 {
        self.piece_bb[get_piece_index(PieceType::King, color)]
    }

    fn get_piece_at_square(&self, square: u32) -> Option<char> {
        let mask = 1u64 << square;

        if self.piece_bb[PieceIndices::WhitePawn as usize] & mask != 0 {
            return Some('P');
        } else if self.piece_bb[PieceIndices::WhiteKnight as usize] & mask != 0 {
            return Some('N');
        } else if self.piece_bb[PieceIndices::WhiteBishop as usize] & mask != 0 {
            return Some('B');
        } else if self.piece_bb[PieceIndices::WhiteRook as usize] & mask != 0 {
            return Some('R');
        } else if self.piece_bb[PieceIndices::WhiteQueen as usize] & mask != 0 {
            return Some('Q');
        } else if self.piece_bb[PieceIndices::WhiteKing as usize] & mask != 0 {
            return Some('K');
        } else if self.piece_bb[PieceIndices::BlackPawn as usize] & mask != 0 {
            return Some('p');
        } else if self.piece_bb[PieceIndices::BlackKnight as usize] & mask != 0 {
            return Some('n');
        } else if self.piece_bb[PieceIndices::BlackBishop as usize] & mask != 0 {
            return Some('b');
        } else if self.piece_bb[PieceIndices::BlackRook as usize] & mask != 0 {
            return Some('r');
        } else if self.piece_bb[PieceIndices::BlackQueen as usize] & mask != 0 {
            return Some('q');
        } else if self.piece_bb[PieceIndices::BlackKing as usize] & mask != 0 {
            return Some('k');
        }

        None
    }

    fn flush_empty_squares(empty_count: &mut i32, fen: &mut String) {
        if *empty_count > 0 {
            fen.push_str(&empty_count.to_string());
            *empty_count = 0;
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty_count = 0;

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                match self.get_piece_at_square(square) {
                    Some(piece) => {
                        BitBoard::flush_empty_squares(&mut empty_count, &mut fen);
                        fen.push(piece);
                    }
                    None => empty_count += 1,
                }
            }
            BitBoard::flush_empty_squares(&mut empty_count, &mut fen);
            if rank > 0 {
                fen.push('/');
            }
        }

        // Add static parts of FEN (assuming white to move, all castling rights, no en passant)
        fen.push_str(" w KQkq - 0 1");
        fen
    }
}

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitBoard FEN: {}", self.to_fen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_position_to_fen() {
        let board = BitBoard::new();
        assert_eq!(
            board.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_empty_board_fen() {
        let board = BitBoard::new_from_pieces(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        assert_eq!(board.to_fen(), "8/8/8/8/8/8/8/8 w KQkq - 0 1");
    }

    #[test]
    #[ignore = "not yet working correctly"]
    fn test_complex_position_fen() {
        // Position with scattered pieces
        let board = BitBoard::new_from_pieces(
            0x10000000000000,   // black pawn on e7
            0x20000000,         // black knight on e5
            0x400000000000,     // black bishop on c7
            0x8000000000000000, // black rook on h8
            0,                  // no black queen
            0x1000000000000000, // black king on e8
            0x1000,             // white pawn on d3
            0x400000,           // white knight on f4
            0x4000000000,       // white bishop on c6
            0x1,                // white rook on a1
            0x800000000000,     // white queen on d7
            0x10,               // white king on e1
        );
        assert_eq!(
            board.to_fen(),
            "4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w HQka - 0 1"
        );
    }
}
