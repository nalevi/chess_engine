use crate::bitboard::BitBoard;
use crate::bitboard::Color;
use crate::bitboard::PieceType;
use rand::prelude::IndexedRandom;

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
}

pub fn pseudo_move(bit_board: &mut BitBoard, move_obj: &Move) {
    bit_board.move_piece(
        &move_obj.piece,
        &move_obj.color,
        move_obj.from as u32,
        move_obj.to as u32,
    );
}

pub fn move_gen(bit_board: &mut BitBoard, color: Color) {
    let moves = collect_all_possible_moves(bit_board, color);

    let valid_moves = get_valid_moves(&moves, bit_board, color);

    // Choose a move (for now, it is just random from the moves array)
    // TODO: Choose the highest value move, when it is implemented
    let random_move = valid_moves.choose(&mut rand::rng()).unwrap();

    // Apply the move to the bitboard
    pseudo_move(bit_board, random_move);
}

fn get_valid_moves(moves: &[Move], bit_board: &BitBoard, color: Color) -> Vec<Move> {
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
            let possible_moves = get_possible_moves_for_piece(piece_type, color, from);

            for to in possible_moves {
                moves.push(Move::new(from, to, piece_type, color));
            }
        }
    });

    moves
}

fn get_possible_moves_for_piece(piece_type: PieceType, color: Color, from: u8) -> Vec<u8> {
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
        let mut to = from as i8 + dir;
        while (to >= 0 && to < 64) && (to % 8 != 0) && (to % 8 != 7) {
            moves.push(to as u8);
            to += dir;
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
        let mut bitboard = BitBoard::new(
            0xFF00, // white pawns
            0, 0, 0, 0, 0, // other white pieces
            0, 0, 0, 0, 0, 0, // black pieces
        );

        let mv = Move::new(8, 16, PieceType::Pawn, Color::White);
        pseudo_move(&mut bitboard, &mv);
        assert_eq!(bitboard.get_pawns(Color::White), 0x1FE00);
    }

    #[test]
    fn test_pseudo_move_blackknight() {
        let mut bitboard = BitBoard::new(
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
        );

        let mv = Move::new(57, 42, PieceType::Knight, Color::Black);
        pseudo_move(&mut bitboard, &mv);
        assert_eq!(bitboard.get_knights(Color::Black), 0x4000040000000000);
    }

    // #[test]
    // fn test_pseudo_move_rook() {
    //     let mut bitboard = BitBoard::new(
    //         0,                  // white pawns
    //         0,                  // white knights
    //         0,                  // white bishops
    //         0x8100000000000000, // white rooks
    //         0,
    //         0, // other white pieces
    //         0,
    //         0,
    //         0,
    //         0,
    //         0,
    //         0, // black pieces
    //     );

    //     let mv = Move::new(63, 55, PieceType::Rook, Color::White);
    //     pseudo_move(&mut bitboard, &mv);

    //     assert_eq!(bitboard.white_rooks, 0x8000000000000000 | (1 << 55));
    //     assert_eq!(bitboard.white_rooks & (1 << 63), 0);
    // }

    // #[test]
    // fn test_pseudo_move_queen() {
    //     let mut bitboard = BitBoard::new(
    //         0,                  // white pawns
    //         0,                  // white knights
    //         0,                  // white bishops
    //         0,                  // white rooks
    //         0x0800000000000000, // white queen
    //         0,                  // white king
    //         0,
    //         0,
    //         0,
    //         0,
    //         0,
    //         0, // black pieces
    //     );

    //     let mv = Move::new(59, 51, PieceType::Queen, Color::White);
    //     pseudo_move(&mut bitboard, &mv);

    //     assert_eq!(bitboard.white_queen, 0x0000000000000000 | (1 << 51));
    //     assert_eq!(bitboard.white_queen & (1 << 59), 0);
    // }

    // #[test]
    // fn test_pseudo_move_king() {
    //     let mut bitboard = BitBoard::new(
    //         0,                  // white pawns
    //         0,                  // white knights
    //         0,                  // white bishops
    //         0,                  // white rooks
    //         0,                  // white queen
    //         0x1000000000000000, // white king
    //         0,
    //         0,
    //         0,
    //         0,
    //         0,
    //         0, // black pieces
    //     );

    //     let mv = Move::new(60, 52, PieceType::King, Color::White);
    //     pseudo_move(&mut bitboard, &mv);

    //     assert_eq!(bitboard.white_king, 0x0000000000000000 | (1 << 52));
    //     assert_eq!(bitboard.white_king & (1 << 60), 0);
    // }
}
