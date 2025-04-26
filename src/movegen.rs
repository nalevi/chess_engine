use crate::bitboard::BitBoard;
use crate::bitboard::Color;
use crate::bitboard::PieceType;
use rand::prelude::IndexedRandom;

#[derive(Clone)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: PieceType,
    pub color: Color,
}

impl Move {
    pub fn new(from: u8, to: u8, piece: PieceType, color: Color) -> Self {
        Move {
            from,
            to,
            piece,
            color,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.piece.to_string(), self.from, self.to)
    }

    pub fn from_string(move_str: &str, color: Color) -> Self {
        let piece = match move_str.chars().nth(0).unwrap() {
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => PieceType::Pawn,
        };

        let mut from_file_idx: usize = 1;
        let mut from_rank_idx: usize = 2;
        let mut to_file_idx: usize = 3;
        let mut to_rank_idx: usize = 4;

        if piece == PieceType::Pawn {
            from_file_idx -= 1;
            from_rank_idx -= 1;
            to_file_idx -= 1;
            to_rank_idx -= 1;
        }

        let from_file = move_str.chars().nth(from_file_idx).unwrap() as u8 - b'a';
        let from_rank = move_str
            .chars()
            .nth(from_rank_idx)
            .unwrap()
            .to_digit(10)
            .unwrap() as u8;
        let to_file = move_str.chars().nth(to_file_idx).unwrap() as u8 - b'a';
        let to_rank = move_str
            .chars()
            .nth(to_rank_idx)
            .unwrap()
            .to_digit(10)
            .unwrap() as u8;

        let from = BitBoard::generate_position_index(from_file, from_rank - 1);
        let to = BitBoard::generate_position_index(to_file, to_rank - 1);

        Move::new(from, to, piece, color)
    }
}

pub fn execute_move(bit_board: &mut BitBoard, move_obj: &Move) {
    bit_board.move_piece(
        &move_obj.piece,
        &move_obj.color,
        move_obj.from as u32,
        move_obj.to as u32,
    );
}

pub fn move_gen(bit_board: &mut BitBoard, color: Color) -> Move {
    let pseudo_moves = collect_all_possible_moves(bit_board, color);

    let valid_moves = get_valid_moves(&pseudo_moves, bit_board, color);

    // Choose a move (for now, it is just random from the moves array)
    // TODO: Choose the highest value move, when it is implemented
    let random_move = valid_moves.choose(&mut rand::rng()).unwrap();

    random_move.clone()
}

fn get_valid_moves(_moves: &[Move], _bit_board: &BitBoard, _color: Color) -> Vec<Move> {
    let valid_moves = Vec::new();

    // TODO: Implement the logic to filter valid moves
    valid_moves
}

fn collect_all_possible_moves(bit_board: &BitBoard, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();

    // Collect all possible moves for each piece type
    PieceType::all().for_each(|piece_type| {
        let piece_bb = bit_board.get_piece_bb(piece_type, color);
        let piece_positions = get_piece_positions(piece_bb);

        for from in piece_positions {
            let pseudo_moves = get_pseudo_moves_for_piece(piece_type, color, from);

            for to in pseudo_moves {
                moves.push(Move::new(from, to, piece_type, color));
            }
        }
    });

    moves
}

fn get_pseudo_moves_for_piece(piece_type: PieceType, color: Color, from: u8) -> Vec<u8> {
    match piece_type {
        PieceType::Pawn => get_pawn_moves(from, color),
        PieceType::Knight => get_knight_moves(from),
        PieceType::Bishop => get_diagonal_moves(from),
        PieceType::Rook => get_sliding_moves(from),
        PieceType::Queen => get_sliding_moves(from)
            .into_iter()
            .chain(get_diagonal_moves(from))
            .collect(),
        PieceType::King => get_king_moves(from),
    }
}

fn get_pawn_moves(from: u8, color: Color) -> Vec<u8> {
    let mut moves = Vec::new();

    if color == Color::Black && from > 7 {
        moves.push(from - 8);
    }

    // Pawns can capture diagonally and move forward one square
    if color == Color::White && from < 56 {
        moves.push(from + 8);
        if from % 8 != 0 {
            moves.push(from + 7); // Capture left
        }
        if from % 8 != 7 {
            moves.push(from + 9); // Capture right
        }
    }

    if color == Color::Black && from > 7 {
        if from % 8 != 0 {
            moves.push(from - 9); // Capture left
        }
        if from % 8 != 7 {
            moves.push(from - 7); // Capture right
        }
    }
    // Pawns can move forward two squares from their starting position
    if from < 16 && color == Color::White {
        moves.push(from + 16);
    }

    if from > 47 && color == Color::Black {
        moves.push(from - 16);
    }

    moves
}

fn get_knight_moves(from: u8) -> Vec<u8> {
    let mut moves = Vec::new();
    let knight_moves = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for &(dx, dy) in &knight_moves {
        let to = from as i8 + dx * 8 + dy;
        if to >= 0 && to < 64 {
            moves.push(to as u8);
        }
    }

    moves
}

fn get_king_moves(from: u8) -> Vec<u8> {
    let mut moves = Vec::new();
    if from > 7 {
        moves.push(from - 8);
    }
    if from < 56 {
        moves.push(from + 8);
    }
    if from % 8 != 0 {
        moves.push(from - 1);
    }
    if from % 8 != 7 {
        moves.push(from + 1);
    }
    if from > 8 && from % 8 != 0 {
        moves.push(from - 9);
    }
    if from > 7 && from % 8 != 7 {
        moves.push(from - 7);
    }
    if from < 56 && from % 8 != 0 {
        moves.push(from + 7);
    }
    if from < 55 && from % 8 != 7 {
        moves.push(from + 9);
    }

    moves
}

fn get_diagonal_moves(from: u8) -> Vec<u8> {
    let mut moves = Vec::new();
    let directions = [7, 9, -7, -9];

    for &dir in &directions {
        let mut to = from as i8;
        while to >= 0 && to < 64 {
            if to % 8 == 0 && (dir == 7 || dir == -9) || to % 8 == 7 && (dir == 9 || dir == -7) {
                break; // Prevent wrapping around the board
            }

            if to < 8 && dir < 0 {
                break; // Prevent moving off the board
            }

            if to > 55 && dir > 0 {
                break; // Prevent moving off the board
            }

            to += dir;
            moves.push(to as u8);
        }
    }

    moves
}

fn get_sliding_moves(from: u8) -> Vec<u8> {
    let mut moves = Vec::new();
    let directions = [1, -1, 8, -8];

    for &dir in &directions {
        let mut to = from as i8;
        while to >= 0 && to < 64 {
            if to % 8 == 0 && dir == -1 || to % 8 == 7 && dir == 1 {
                break; // Prevent wrapping around the board
            }

            if to < 8 && dir < 0 {
                break; // Prevent moving off the board
            }

            if to > 55 && dir > 0 {
                break; // Prevent moving off the board
            }

            to += dir;
            moves.push(to as u8);
        }
    }

    moves
}

fn get_piece_positions(piece_bb: u64) -> Vec<u8> {
    let mut positions = Vec::new();
    let bb = piece_bb;

    for i in 0..64 {
        if bb & (1 << i) != 0 {
            positions.push(i as u8);
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pseudo_move_whitepawn_forward() {
        let mut bitboard = BitBoard::new_from_pieces(
            0xFF00, // white pawns
            0,
            0,
            0,
            0,
            0, // other white pieces
            0,
            0,
            0,
            0,
            0,
            0, // black pieces
            Color::Black,
            0,
            None,
            0,
            1,
        );

        let mv = Move::new(8, 16, PieceType::Pawn, Color::White);
        execute_move(&mut bitboard, &mv);
        assert_eq!(bitboard.get_pawns(Color::White), 0x1FE00);
    }

    #[test]
    fn test_pseudo_move_blackknight() {
        let mut bitboard = BitBoard::new_from_pieces(
            0, // white pawns
            0, // white knights
            0,
            0,
            0,
            0, // other white pieces
            0,
            0x4200000000000000, // black knights
            0,
            0,
            0,
            0, // black pieces
            Color::Black,
            0,
            None,
            0,
            1,
        );
        let mv = Move::new(57, 42, PieceType::Knight, Color::Black);
        execute_move(&mut bitboard, &mv);

        assert_eq!(bitboard.get_knights(Color::Black), 0x4000040000000000);
    }

    #[test]
    fn test_get_diagonal_moves() {
        // Test from a corner (top-left)
        let moves = get_diagonal_moves(0);
        assert_eq!(moves, vec![9, 18, 27, 36, 45, 54, 63]);

        // Test from a corner (bottom-right)
        let moves = get_diagonal_moves(63);
        assert_eq!(moves, vec![54, 45, 36, 27, 18, 9, 0]);

        // Test from the center of the board
        let moves = get_diagonal_moves(27);
        assert_eq!(moves, vec![34, 41, 48, 36, 45, 54, 63, 20, 13, 6, 18, 9, 0]);

        // Test from an edge (left edge, not corner)
        let moves = get_diagonal_moves(8);
        assert_eq!(moves, vec![17, 26, 35, 44, 53, 62, 1]);

        // Test from an edge (right edge, not corner)
        let moves = get_diagonal_moves(15);
        assert_eq!(moves, vec![22, 29, 36, 43, 50, 57, 6]);
    }

    #[test]
    fn test_move_from_string_pawn() {
        let mv = Move::from_string("e2e4", Color::White);
        assert_eq!(mv.from, 12);
        assert_eq!(mv.to, 28);
        assert_eq!(mv.piece, PieceType::Pawn);
        assert_eq!(mv.color, Color::White);
    }

    #[test]
    fn test_move_from_string_knight() {
        let mv = Move::from_string("Nb8c6", Color::Black);
        assert_eq!(mv.from, 57);
        assert_eq!(mv.to, 42);
        assert_eq!(mv.piece, PieceType::Knight);
        assert_eq!(mv.color, Color::Black);
    }

    #[test]
    fn test_move_from_string_bishop() {
        let mv = Move::from_string("Bf1c4", Color::White);
        assert_eq!(mv.from, 5);
        assert_eq!(mv.to, 26);
        assert_eq!(mv.piece, PieceType::Bishop);
        assert_eq!(mv.color, Color::White);
    }

    #[test]
    fn test_move_from_string_rook() {
        let mv = Move::from_string("Ra8b8", Color::Black);
        assert_eq!(mv.from, 56);
        assert_eq!(mv.to, 57);
        assert_eq!(mv.piece, PieceType::Rook);
        assert_eq!(mv.color, Color::Black);
    }

    #[test]
    fn test_move_from_string_queen() {
        let mv = Move::from_string("Qd1h5", Color::White);
        assert_eq!(mv.from, 3);
        assert_eq!(mv.to, 39);
        assert_eq!(mv.piece, PieceType::Queen);
        assert_eq!(mv.color, Color::White);
    }

    #[test]
    fn test_move_from_string_king() {
        let mv = Move::from_string("Kg1h2", Color::Black);
        assert_eq!(mv.from, 6);
        assert_eq!(mv.to, 15);
        assert_eq!(mv.piece, PieceType::King);
        assert_eq!(mv.color, Color::Black);
    }
}
