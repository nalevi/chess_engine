use std::fmt::Display;

use log::{debug, trace};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    White = 0,
    Black = 6,
}

#[derive(Copy, Clone, PartialEq, Debug)]
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

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "P"),
            PieceType::Knight => write!(f, "N"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::King => write!(f, "K"),
        }
    }
}

impl PieceType {
    pub fn all() -> impl Iterator<Item = PieceType> {
        use PieceType::*;
        [Pawn, Knight, Bishop, Rook, Queen, King].iter().copied()
    }

    pub fn from_char(c: char) -> Option<PieceType> {
        match c.to_uppercase().next().unwrap() {
            'P' => Some(PieceType::Pawn),
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'R' => Some(PieceType::Rook),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
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
    piece_bb: [u64; 12],
    to_move: Color,
    castling_rights: u8,
    en_passant: Option<u8>,
    halfmove_clock: u8,
    fullmove_number: u8,
}

impl BitBoard {
    pub fn new() -> Self {
        debug!("Creating an empty BitBoard");
        BitBoard {
            piece_bb: [0; 12],
            to_move: Color::White,
            castling_rights: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn new_clear_board() -> Self {
        debug!("Creating a new BitBoard with standard starting position");
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
            to_move: Color::White,
            castling_rights: 0b1111, // KQkq
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
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
        to_move: Color,
        castling_rights: u8,
        en_passant: Option<u8>,
        halfmove_clock: u8,
        fullmove_number: u8,
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
            to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        }
    }

    pub fn from_fen(fen_str: &str) -> Self {
        debug!("Creating a BitBoard from FEN: {}", fen_str);
        let mut piece_bb = [0; 12];
        let mut to_move = Color::White;
        let mut castling_rights = 0b0000;
        let mut en_passant_bb: Option<u8> = None;
        let mut halfmove_clock = 0;
        let mut fullmove_number = 1;

        // read the board part, using '/' as a separator
        let mut rank = 7;
        let mut file = 0;
        for c in fen_str.chars() {
            if c == ' ' {
                break;
            }
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
                let empty_squares = c.to_digit(10).unwrap();
                file += empty_squares as u32;
            } else {
                let piece = match c {
                    'P' => PieceType::Pawn,
                    'N' => PieceType::Knight,
                    'B' => PieceType::Bishop,
                    'R' => PieceType::Rook,
                    'Q' => PieceType::Queen,
                    'K' => PieceType::King,
                    'p' => PieceType::Pawn,
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    'k' => PieceType::King,
                    _ => continue,
                };
                let color = if c.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                piece_bb[get_piece_index(piece, color)] |=
                    1u64 << BitBoard::generate_position_index(file as u8, rank);
                file += 1;
            }
        }

        let mut iter = fen_str.split_whitespace();
        iter.next(); // skip the board part
        if let Some(active_color) = iter.next() {
            to_move = if active_color == "w" {
                Color::White
            } else {
                Color::Black
            };
        }

        if let Some(castling) = iter.next() {
            let mut rights = 0b0000;
            if castling.contains('K') {
                rights |= 0b0001;
            }
            if castling.contains('Q') {
                rights |= 0b0010;
            }
            if castling.contains('k') {
                rights |= 0b0100;
            }
            if castling.contains('q') {
                rights |= 0b1000;
            }
            castling_rights = rights;
        }

        if let Some(en_passant) = iter.next() {
            if en_passant != "-" {
                let file = en_passant
                    .chars()
                    .next()
                    .map(|c| c as u8 - b'a')
                    .unwrap_or_else(|| panic!("Invalid en_passant string"));

                let rank = en_passant
                    .chars()
                    .nth(1)
                    .and_then(|c| c.to_digit(10))
                    .map(|d| (d - 1) as u8)
                    .unwrap_or_else(|| panic!("Invalid en_passant string: {}", en_passant));

                en_passant_bb = Some(BitBoard::generate_position_index(file, rank));
            } else {
                en_passant_bb = None;
            }
        }

        if let Some(halfmove) = iter.next() {
            halfmove_clock = halfmove.parse().unwrap_or(0);
        }

        if let Some(fullmove) = iter.next() {
            fullmove_number = fullmove.parse().unwrap_or(1);
        }

        BitBoard {
            piece_bb,
            to_move,
            castling_rights,
            en_passant: en_passant_bb,
            halfmove_clock,
            fullmove_number,
        }
    }

    pub fn move_piece(&mut self, pice_type: &PieceType, color: &Color, from: u32, to: u32) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        self.piece_bb[get_piece_index(*pice_type, *color)] &= !from_mask;
        self.piece_bb[get_piece_index(*pice_type, *color)] |= to_mask;

        if color == &Color::White {
            self.to_move = Color::Black;
        } else {
            self.to_move = Color::White;
            self.fullmove_number += 1;
        }
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

    pub fn get_piece_at_square(&self, square: u32) -> Option<char> {
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
                        trace!("Piece at square {}: {}", square, piece);
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
        // Add the active color
        fen.push(' ');
        fen.push(if self.to_move == Color::White {
            'w'
        } else {
            'b'
        });

        // Add castling rights
        fen.push(' ');
        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if self.castling_rights & 0b0001 != 0 {
                fen.push('K');
            }
            if self.castling_rights & 0b0010 != 0 {
                fen.push('Q');
            }
            if self.castling_rights & 0b0100 != 0 {
                fen.push('k');
            }
            if self.castling_rights & 0b1000 != 0 {
                fen.push('q');
            }
        }

        // Add en passant
        fen.push(' ');
        if let Some(ep) = self.en_passant {
            let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
            fen.push(files[ep as usize % 8]);
            let rank = ep / 8 + 1;
            fen.push(rank.to_string().chars().next().unwrap());
        } else {
            fen.push('-');
        }

        // Add halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        // Add fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    pub fn generate_position_index(file: u8, rank: u8) -> u8 {
        (rank * 8 + file) as u8
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
        let board = BitBoard::new_clear_board();
        assert_eq!(
            board.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_empty_board_to_fen() {
        let board = BitBoard::new();
        assert_eq!(board.to_fen(), "8/8/8/8/8/8/8/8 w - - 0 1");
    }

    #[test]
    fn test_complex_position_to_fen() {
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
            Color::White,       // white to move
            0b1111,             // all castling rights
            None,               // no en passant
            0,
            1,
        );
        assert_eq!(
            board.to_fen(),
            "4K2R/4P3/6Bq/6b1/5N2/6n1/4p3/r3k3 w KQkq - 0 1"
        );
    }

    #[test]
    fn test_complex_position_to_fen_with_enpassant() {
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
            Color::White,       // white to move
            0b1111,             // all castling rights
            Some(15),           // en passant
            0,
            1,
        );
        assert_eq!(
            board.to_fen(),
            "4K2R/4P3/6Bq/6b1/5N2/6n1/4p3/r3k3 w KQkq h2 0 1"
        );
    }

    #[test]
    fn test_from_fen_initial_position() {
        let board = BitBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(
            board.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_from_fen_empty_board() {
        let board = BitBoard::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");
        assert_eq!(board.to_fen(), "8/8/8/8/8/8/8/8 w - - 0 1");
    }

    #[test]
    fn test_from_fen_complex_position() {
        let board = BitBoard::from_fen("4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w KQkq - 0 1");
        assert_eq!(
            board.to_fen(),
            "4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w KQkq - 0 1"
        );
    }

    #[test]
    fn test_from_fen_with_en_passant() {
        let board = BitBoard::from_fen("4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w KQkq e6 0 1");
        assert_eq!(
            board.to_fen(),
            "4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w KQkq e6 0 1"
        );
    }

    #[test]
    fn test_from_fen_no_castling_rights() {
        let board = BitBoard::from_fen("4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w - - 0 1");
        assert_eq!(
            board.to_fen(),
            "4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w - - 0 1"
        );
    }

    #[test]
    fn test_from_fen_white_castling_rights_queen_side() {
        let board = BitBoard::from_fen("4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w Q - 0 1");
        assert_eq!(
            board.to_fen(),
            "4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 w Q - 0 1"
        );
    }

    #[test]
    fn test_from_fen_black_to_move() {
        let board = BitBoard::from_fen("4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 b KQkq - 0 1");
        assert_eq!(
            board.to_fen(),
            "4k2r/4p3/6bQ/6B1/5n2/6N1/4P3/R3K3 b KQkq - 0 1"
        );
    }
}
