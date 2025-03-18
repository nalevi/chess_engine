use crate::bitboard::BitBoard;
use crate::bitboard::Color;
use crate::bitboard::PieceType;

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
