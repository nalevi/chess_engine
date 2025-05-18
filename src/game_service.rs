use crate::bitboard::Color;

use crate::bitboard::BitBoard;
use crate::bitboard::PieceType;

pub struct GameService {
    board: BitBoard,
}

impl GameService {
    pub fn new() -> Self {
        GameService {
            board: BitBoard::new_clear_board(),
        }
    }

    pub fn init_game_from_position(&mut self, fen_str: &str) {
        self.board = BitBoard::from_fen(fen_str);
    }

    pub fn reset_game(&mut self) {
        self.board = BitBoard::new_clear_board();
    }

    /// This function executes moves on the bitboard.
    /// The moves are in simple UCI formats, <from><to>, e.g.: e2e4.
    pub fn feed_moves_to_game_board(&mut self, moves: Vec<String>) {
        let mut color = Color::White;
        for m in moves {
            let from_file = m.chars().nth(1).unwrap() as u8 - b'a';
            let from_rank = m.chars().nth(2).unwrap().to_digit(10).unwrap() as u8;

            let to_file = m.chars().nth(3).unwrap() as u8 - b'a';
            let to_rank = m.chars().nth(4).unwrap().to_digit(10).unwrap() as u8;

            let from = BitBoard::generate_position_index(from_file, from_rank);
            let to = BitBoard::generate_position_index(to_file, to_rank);

            let piece = self.board.get_piece_at_square(from as u32).unwrap();
            let piece_type = PieceType::from_char(piece).unwrap();

            BitBoard::move_piece(&mut self.board, &piece_type, &color, from as u32, to as u32);
            color = if color == Color::White {
                Color::Black
            } else {
                Color::White
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initial_position_fen() {
        let gs = GameService::new();
        assert_eq!(
            gs.board.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_init_game_from_position() {
        let mut gs = GameService::new();
        let custom_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        gs.init_game_from_position(custom_fen);
        assert_eq!(gs.board.to_fen(), custom_fen);
    }
}
