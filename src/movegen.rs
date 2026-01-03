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
        let from_file = (self.from % 8 + b'a') as char;
        let from_rank = self.from / 8 + 1;
        let to_file = (self.to % 8 + b'a') as char;
        let to_rank = self.to / 8 + 1;
        format!("{}{}{}{}", from_file, from_rank, to_file, to_rank)
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

pub fn move_gen(bit_board: &mut BitBoard) -> Move {
    let pseudo_moves = collect_all_possible_moves(bit_board, bit_board.get_current_color());

    let valid_moves = get_valid_moves(&pseudo_moves, bit_board, bit_board.get_current_color());

    // Choose a move (for now, it is just random from the moves array)
    // TODO: Choose the highest value move, when it is implemented
    let random_move = valid_moves.choose(&mut rand::rng()).unwrap();

    random_move.clone()
}

fn get_valid_moves(moves: &[Move], bit_board: &BitBoard, color: Color) -> Vec<Move> {
    let mut valid_moves = Vec::new();

    for m in moves {
        // Cannot move to a square occupied by the same color
        if bit_board.is_occupied_by_color(m.to as u32, color) {
            continue;
        }

        // Additional validation for pawn moves
        if m.piece == PieceType::Pawn {
            let is_diagonal = (m.from as i8 - m.to as i8).abs() % 8 != 0;

            if is_diagonal {
                // Pawns can only move diagonally to capture (must have enemy piece)
                if bit_board.get_piece_at_square(m.to as u32).is_none() {
                    continue; // Cannot move diagonally without capturing
                }
            } else {
                // Pawns moving forward must have empty target square
                if bit_board.get_piece_at_square(m.to as u32).is_some() {
                    continue; // Cannot move forward into a piece
                }
            }
        }

        valid_moves.push(m.clone());
    }
    valid_moves
}

fn collect_all_possible_moves(bit_board: &BitBoard, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();

    // Collect all possible moves for each piece type
    PieceType::all().for_each(|piece_type| {
        let piece_bb = bit_board.get_piece_bb(piece_type, color);
        let piece_positions = get_piece_positions(piece_bb);

        for from in piece_positions {
            let pseudo_moves = get_pseudo_moves_for_piece(piece_type, color, from, bit_board);

            for to in pseudo_moves {
                moves.push(Move::new(from, to, piece_type, color));
            }
        }
    });

    moves
}

fn get_pseudo_moves_for_piece(piece_type: PieceType, color: Color, from: u8, bit_board: &BitBoard) -> Vec<u8> {
    match piece_type {
        PieceType::Pawn => get_pawn_moves(from, color, bit_board),
        PieceType::Knight => get_knight_moves(from, bit_board, color),
        PieceType::Bishop => get_diagonal_moves(from, bit_board, color),
        PieceType::Rook => get_sliding_moves(from, bit_board, color),
        PieceType::Queen => {
            let mut moves = get_sliding_moves(from, bit_board, color);
            moves.extend(get_diagonal_moves(from, bit_board, color));
            moves
        },
        PieceType::King => get_king_moves(from, bit_board, color),
    }
}

fn get_pawn_moves(from: u8, color: Color, bit_board: &BitBoard) -> Vec<u8> {
    let mut moves = Vec::new();

    if color == Color::White {
        // Forward one square
        if from < 56 {
            let forward_one = from + 8;
            if bit_board.get_piece_at_square(forward_one as u32).is_none() {
                moves.push(forward_one);
                
                // Forward two squares from starting position
                if from < 16 {
                    let forward_two = from + 16;
                    let intermediate = forward_one;
                    // Check that intermediate square is also empty
                    if bit_board.get_piece_at_square(intermediate as u32).is_none() 
                        && bit_board.get_piece_at_square(forward_two as u32).is_none() {
                        moves.push(forward_two);
                    }
                }
            }
        }
        
        // Diagonal captures (only if enemy piece present)
        if from < 56 {
            if from % 8 != 0 {
                let capture_left = from + 7;
                if let Some(piece) = bit_board.get_piece_at_square(capture_left as u32) {
                    if piece.is_ascii_lowercase() {
                        moves.push(capture_left);
                    }
                }
            }
            if from % 8 != 7 {
                let capture_right = from + 9;
                if let Some(piece) = bit_board.get_piece_at_square(capture_right as u32) {
                    if piece.is_ascii_lowercase() {
                        moves.push(capture_right);
                    }
                }
            }
        }
    } else {
        // Black pawns
        // Forward one square
        if from > 7 {
            let forward_one = from - 8;
            if bit_board.get_piece_at_square(forward_one as u32).is_none() {
                moves.push(forward_one);
                
                // Forward two squares from starting position
                if from > 47 {
                    let forward_two = from - 16;
                    let intermediate = forward_one;
                    // Check that intermediate square is also empty
                    if bit_board.get_piece_at_square(intermediate as u32).is_none() 
                        && bit_board.get_piece_at_square(forward_two as u32).is_none() {
                        moves.push(forward_two);
                    }
                }
            }
        }
        
        // Diagonal captures (only if enemy piece present)
        if from > 7 {
            if from % 8 != 0 {
                let capture_left = from - 9;
                if let Some(piece) = bit_board.get_piece_at_square(capture_left as u32) {
                    if piece.is_ascii_uppercase() {
                        moves.push(capture_left);
                    }
                }
            }
            if from % 8 != 7 {
                let capture_right = from - 7;
                if let Some(piece) = bit_board.get_piece_at_square(capture_right as u32) {
                    if piece.is_ascii_uppercase() {
                        moves.push(capture_right);
                    }
                }
            }
        }
    }

    moves
}

fn get_knight_moves(from: u8, bit_board: &BitBoard, color: Color) -> Vec<u8> {
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
            let to_square = to as u8;
            // Knights can't move to squares occupied by same color
            if !bit_board.is_occupied_by_color(to_square as u32, color) {
                moves.push(to_square);
            }
        }
    }

    moves
}

fn get_king_moves(from: u8, bit_board: &BitBoard, color: Color) -> Vec<u8> {
    let mut moves = Vec::new();
    
    let candidates = vec![
        if from > 7 { Some(from - 8) } else { None },
        if from < 56 { Some(from + 8) } else { None },
        if from % 8 != 0 { Some(from - 1) } else { None },
        if from % 8 != 7 { Some(from + 1) } else { None },
        if from > 8 && from % 8 != 0 { Some(from - 9) } else { None },
        if from > 7 && from % 8 != 7 { Some(from - 7) } else { None },
        if from < 56 && from % 8 != 0 { Some(from + 7) } else { None },
        if from < 55 && from % 8 != 7 { Some(from + 9) } else { None },
    ];

    for candidate in candidates {
        if let Some(to) = candidate {
            // King can't move to squares occupied by same color
            if !bit_board.is_occupied_by_color(to as u32, color) {
                moves.push(to);
            }
        }
    }

    moves
}

fn get_diagonal_moves(from: u8, bit_board: &BitBoard, color: Color) -> Vec<u8> {
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
            if to < 0 || to >= 64 {
                break;
            }

            let to_square = to as u8;
            
            // Check if square is occupied
            if let Some(piece) = bit_board.get_piece_at_square(to_square as u32) {
                // If enemy piece, can capture and stop
                let is_enemy = (color == Color::White && piece.is_ascii_lowercase())
                    || (color == Color::Black && piece.is_ascii_uppercase());
                if is_enemy {
                    moves.push(to_square);
                }
                // Stop in this direction (blocked by piece)
                break;
            } else {
                // Empty square, can move here and continue
                moves.push(to_square);
            }
        }
    }

    moves
}

fn get_sliding_moves(from: u8, bit_board: &BitBoard, color: Color) -> Vec<u8> {
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
            if to < 0 || to >= 64 {
                break;
            }

            let to_square = to as u8;
            
            // Check if square is occupied
            if let Some(piece) = bit_board.get_piece_at_square(to_square as u32) {
                // If enemy piece, can capture and stop
                let is_enemy = (color == Color::White && piece.is_ascii_lowercase())
                    || (color == Color::Black && piece.is_ascii_uppercase());
                if is_enemy {
                    moves.push(to_square);
                }
                // Stop in this direction (blocked by piece)
                break;
            } else {
                // Empty square, can move here and continue
                moves.push(to_square);
            }
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
        // Create an empty board for testing (no blocking pieces)
        let bitboard = BitBoard::new();
        
        // Test from a corner (top-left)
        let moves = get_diagonal_moves(0, &bitboard, Color::White);
        assert_eq!(moves, vec![9, 18, 27, 36, 45, 54, 63]);

        // Test from a corner (bottom-right)
        let moves = get_diagonal_moves(63, &bitboard, Color::White);
        assert_eq!(moves, vec![54, 45, 36, 27, 18, 9, 0]);

        // Test from the center of the board
        let moves = get_diagonal_moves(27, &bitboard, Color::White);
        assert_eq!(moves, vec![34, 41, 48, 36, 45, 54, 63, 20, 13, 6, 18, 9, 0]);

        // Test from an edge (left edge, not corner)
        let moves = get_diagonal_moves(8, &bitboard, Color::White);
        assert_eq!(moves, vec![17, 26, 35, 44, 53, 62, 1]);

        // Test from an edge (right edge, not corner)
        let moves = get_diagonal_moves(15, &bitboard, Color::White);
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

    // Pawn move generator tests
    #[test]
    fn test_get_pawn_moves_white_forward_one() {
        let bitboard = BitBoard::new();
        let moves = get_pawn_moves(12, Color::White, &bitboard); // e2
        assert!(moves.contains(&20)); // e3
    }

    #[test]
    fn test_get_pawn_moves_white_forward_two_from_start() {
        let bitboard = BitBoard::new();
        let moves = get_pawn_moves(12, Color::White, &bitboard); // e2
        assert!(moves.contains(&20)); // e3
        assert!(moves.contains(&28)); // e4
    }

    #[test]
    fn test_get_pawn_moves_white_blocked_forward() {
        // White pawn at e2, black pawn at e3
        let bitboard = BitBoard::new_from_pieces(
            0x1000, // white pawn at e2 (square 12)
            0, 0, 0, 0, 0,
            0x100000, // black pawn at e3 (square 20)
            0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_pawn_moves(12, Color::White, &bitboard);
        assert!(!moves.contains(&20)); // Cannot move forward (blocked)
        assert!(!moves.contains(&28)); // Cannot move two squares (blocked)
    }

    #[test]
    fn test_get_pawn_moves_white_diagonal_capture() {
        // White pawn at e2, black pawn at d3
        let bitboard = BitBoard::new_from_pieces(
            0x1000, // white pawn at e2 (square 12)
            0, 0, 0, 0, 0,
            0x80000, // black pawn at d3 (square 19)
            0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_pawn_moves(12, Color::White, &bitboard);
        assert!(moves.contains(&19)); // Can capture diagonally left
    }

    #[test]
    fn test_get_pawn_moves_white_no_diagonal_without_capture() {
        let bitboard = BitBoard::new();
        let moves = get_pawn_moves(12, Color::White, &bitboard); // e2
        assert!(!moves.contains(&19)); // Cannot move diagonally without capture
        assert!(!moves.contains(&21)); // Cannot move diagonally without capture
    }

    #[test]
    fn test_get_pawn_moves_black_forward_one() {
        let bitboard = BitBoard::new();
        let moves = get_pawn_moves(52, Color::Black, &bitboard); // e7
        assert!(moves.contains(&44)); // e6
    }

    #[test]
    fn test_get_pawn_moves_black_forward_two_from_start() {
        let bitboard = BitBoard::new();
        let moves = get_pawn_moves(52, Color::Black, &bitboard); // e7
        assert!(moves.contains(&44)); // e6
        assert!(moves.contains(&36)); // e5
    }

    #[test]
    fn test_get_pawn_moves_black_blocked_forward() {
        // Black pawn at e7 (square 52), white pawn at e6 (square 44)
        let bitboard = BitBoard::new_from_pieces(
            0x100000000000, // white pawn at e6 (square 44)
            0, 0, 0, 0, 0,
            0x10000000000000, // black pawn at e7 (square 52)
            0, 0, 0, 0, 0,
            Color::Black, 0, None, 0, 1,
        );
        let moves = get_pawn_moves(52, Color::Black, &bitboard);
        assert!(!moves.contains(&44)); // Cannot move forward (blocked)
        assert!(!moves.contains(&36)); // Cannot move two squares (blocked)
    }

    #[test]
    fn test_get_pawn_moves_black_diagonal_capture() {
        // Black pawn at e7 (square 52), white pawn at d6 (square 43)
        let bitboard = BitBoard::new_from_pieces(
            0x80000000000, // white pawn at d6 (square 43)
            0, 0, 0, 0, 0,
            0x10000000000000, // black pawn at e7 (square 52)
            0, 0, 0, 0, 0,
            Color::Black, 0, None, 0, 1,
        );
        let moves = get_pawn_moves(52, Color::Black, &bitboard);
        assert!(moves.contains(&43)); // Can capture diagonally left
    }

    // Knight move generator tests
    #[test]
    fn test_get_knight_moves_from_center() {
        let bitboard = BitBoard::new();
        let moves = get_knight_moves(27, &bitboard, Color::White); // d4
        // Knight from d4 should have 8 possible moves
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&10)); // b3
        assert!(moves.contains(&12)); // c2
        assert!(moves.contains(&17)); // b5
        assert!(moves.contains(&21)); // c6
        assert!(moves.contains(&33)); // e6
        assert!(moves.contains(&37)); // f5
        assert!(moves.contains(&42)); // f3
        assert!(moves.contains(&44)); // e2
    }

    #[test]
    fn test_get_knight_moves_from_corner() {
        let bitboard = BitBoard::new();
        let moves = get_knight_moves(0, &bitboard, Color::White); // a8
        // Knight from corner a8 can move to: b6, h7, c7, g8
        assert_eq!(moves.len(), 4);
        assert!(moves.contains(&10)); // b6
        assert!(moves.contains(&15)); // h7
        assert!(moves.contains(&17)); // c7
        assert!(moves.contains(&6)); // g8
    }

    #[test]
    fn test_get_knight_moves_blocked_by_same_color() {
        // White knight at d4 (square 27), white pawn at e6 (square 44)
        let bitboard = BitBoard::new_from_pieces(
            0x100000000000, // white pawn at e6 (square 44)
            0x8000000, // white knight at d4 (square 27)
            0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_knight_moves(27, &bitboard, Color::White);
        assert!(!moves.contains(&44)); // Cannot move to square occupied by same color
    }

    #[test]
    fn test_get_knight_moves_can_capture_enemy() {
        // White knight at d4 (square 27), black pawn at e6 (square 44)
        let bitboard = BitBoard::new_from_pieces(
            0,
            0x8000000, // white knight at d4 (square 27)
            0, 0, 0, 0,
            0x100000000000, // black pawn at e6 (square 44)
            0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_knight_moves(27, &bitboard, Color::White);
        assert!(moves.contains(&44)); // Can capture enemy piece
    }

    // King move generator tests
    #[test]
    fn test_get_king_moves_from_center() {
        let bitboard = BitBoard::new();
        let moves = get_king_moves(27, &bitboard, Color::White); // d4
        // King from center should have 8 possible moves
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_get_king_moves_from_corner() {
        let bitboard = BitBoard::new();
        let moves = get_king_moves(0, &bitboard, Color::White); // a8
        // King from corner should have 3 possible moves
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&1)); // b8
        assert!(moves.contains(&8)); // a7
        assert!(moves.contains(&9)); // b7
    }

    #[test]
    fn test_get_king_moves_blocked_by_same_color() {
        // White king at e1, white pawn at e2
        let bitboard = BitBoard::new_from_pieces(
            0x1000, // white pawn at e2 (square 12)
            0, 0, 0, 0,
            0x10, // white king at e1 (square 4)
            0, 0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_king_moves(4, &bitboard, Color::White);
        assert!(!moves.contains(&12)); // Cannot move to square occupied by same color
    }

    #[test]
    fn test_get_king_moves_can_capture_enemy() {
        // White king at e1, black pawn at e2
        let bitboard = BitBoard::new_from_pieces(
            0,
            0, 0, 0, 0,
            0x10, // white king at e1 (square 4)
            0x1000, // black pawn at e2 (square 12)
            0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_king_moves(4, &bitboard, Color::White);
        assert!(moves.contains(&12)); // Can capture enemy piece
    }

    // Bishop move generator tests
    #[test]
    fn test_get_diagonal_moves_blocked_by_friendly() {
        // White bishop at c1 (square 2), white pawn at e3 (square 20)
        let bitboard = BitBoard::new_from_pieces(
            0x100000, // white pawn at e3 (square 20)
            0,
            0x4, // white bishop at c1 (square 2)
            0, 0, 0,
            0, 0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_diagonal_moves(2, &bitboard, Color::White); // c1
        // Should not include squares beyond the blocking pawn
        assert!(!moves.contains(&38)); // g5 (beyond blocking pawn at e3)
        assert!(moves.contains(&11)); // b2 (before blocking pawn)
        assert!(!moves.contains(&20)); // Cannot move to square with friendly piece
    }

    #[test]
    fn test_get_diagonal_moves_blocked_by_enemy() {
        // White bishop at c1 (square 2), black pawn at e3 (square 20)
        let bitboard = BitBoard::new_from_pieces(
            0,
            0,
            0x4, // white bishop at c1 (square 2)
            0, 0, 0,
            0x100000, // black pawn at e3 (square 20)
            0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_diagonal_moves(2, &bitboard, Color::White); // c1
        // Should include the enemy piece (can capture) but not beyond
        assert!(moves.contains(&20)); // Can capture at e3
        assert!(!moves.contains(&38)); // Cannot go beyond e3
        assert!(moves.contains(&11)); // Can move to b2
    }

    #[test]
    fn test_get_diagonal_moves_multiple_directions() {
        // White bishop at d4, test all four diagonal directions
        let bitboard = BitBoard::new();
        let moves = get_diagonal_moves(27, &bitboard, Color::White); // d4
        // Should have moves in all four diagonal directions
        assert!(moves.contains(&18)); // b2 (southwest)
        assert!(moves.contains(&20)); // c2 (southeast)
        assert!(moves.contains(&34)); // e6 (northeast)
        assert!(moves.contains(&36)); // f6 (northwest)
    }

    // Rook move generator tests
    #[test]
    fn test_get_sliding_moves_blocked_by_friendly() {
        // White rook at a1 (square 56), white pawn at a3 (square 40)
        let bitboard = BitBoard::new_from_pieces(
            0x10000000000, // white pawn at a3 (square 40)
            0, 0,
            0x100000000000000, // white rook at a1 (square 56)
            0, 0,
            0, 0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_sliding_moves(56, &bitboard, Color::White); // a1
        // Should not include squares beyond the blocking pawn
        assert!(!moves.contains(&32)); // a4 (beyond blocking pawn)
        assert!(moves.contains(&48)); // a2 (before blocking pawn)
        assert!(!moves.contains(&40)); // Cannot move to square with friendly piece
    }

    #[test]
    fn test_get_sliding_moves_blocked_by_enemy() {
        // White rook at a1 (square 56), black pawn at a3 (square 40)
        let bitboard = BitBoard::new_from_pieces(
            0,
            0, 0,
            0x100000000000000, // white rook at a1 (square 56)
            0, 0,
            0x10000000000, // black pawn at a3 (square 40)
            0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_sliding_moves(56, &bitboard, Color::White); // a1
        // Should include the enemy piece (can capture) but not beyond
        assert!(moves.contains(&40)); // Can capture at a3
        assert!(!moves.contains(&32)); // Cannot go beyond a3
        assert!(moves.contains(&48)); // Can move to a2
    }

    #[test]
    fn test_get_sliding_moves_all_directions() {
        // White rook at d4, test all four directions
        let bitboard = BitBoard::new();
        let moves = get_sliding_moves(27, &bitboard, Color::White); // d4
        // Should have moves in all four directions
        assert!(moves.contains(&19)); // d3 (south)
        assert!(moves.contains(&26)); // c4 (west)
        assert!(moves.contains(&28)); // e4 (east)
        assert!(moves.contains(&35)); // d5 (north)
    }

    #[test]
    fn test_get_sliding_moves_horizontal_blocked() {
        // White rook at d4, white pawn at e4
        let bitboard = BitBoard::new_from_pieces(
            0x10000000, // white pawn at e4 (square 28)
            0, 0,
            0x8000000, // white rook at d4 (square 27)
            0, 0,
            0, 0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_sliding_moves(27, &bitboard, Color::White); // d4
        // Should not be able to move east past e4
        assert!(!moves.contains(&29)); // f4 (beyond blocking pawn)
        assert!(!moves.contains(&28)); // Cannot move to square with friendly piece
        // But can still move west
        assert!(moves.contains(&26)); // c4
    }

    // Queen move generator tests
    #[test]
    fn test_get_queen_moves_combines_rook_and_bishop() {
        let bitboard = BitBoard::new();
        let moves = get_pseudo_moves_for_piece(PieceType::Queen, Color::White, 27, &bitboard); // d4
        // Queen should have both rook and bishop moves
        assert!(moves.contains(&19)); // d3 (rook move - south)
        assert!(moves.contains(&26)); // c4 (rook move - west)
        assert!(moves.contains(&18)); // b2 (bishop move - southwest)
        assert!(moves.contains(&34)); // e6 (bishop move - northeast)
    }

    #[test]
    fn test_get_queen_moves_blocked_in_multiple_directions() {
        // White queen at d4 (square 27), white pawns blocking in multiple directions
        // d3 = square 19, e4 = square 28
        let bitboard = BitBoard::new_from_pieces(
            0x10080000, // white pawns at d3 (19) and e4 (28) - 0x80000 | 0x10000000
            0, 0, 0,
            0x8000000, // white queen at d4 (square 27)
            0,
            0, 0, 0, 0, 0, 0,
            Color::White, 0, None, 0, 1,
        );
        let moves = get_pseudo_moves_for_piece(PieceType::Queen, Color::White, 27, &bitboard);
        // Should not include blocked directions
        assert!(!moves.contains(&19)); // Cannot move to d3 (friendly piece)
        assert!(!moves.contains(&28)); // Cannot move to e4 (friendly piece)
        // But can still move in other directions
        assert!(moves.contains(&26)); // c4 (west)
        assert!(moves.contains(&35)); // d5 (north)
    }
}
