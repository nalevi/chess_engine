use crate::bitboard::Color;

use crate::bitboard::BitBoard;
use crate::bitboard::PieceType;

use log::debug;

#[derive(Debug)]
pub enum GameError {
    InvalidMoveFormat,
    NoPieceAtSquare,
    WrongColorPiece,
}

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
        debug!("Current table FEN: {}", self.board.to_fen());
    }

    /// This function executes moves on the bitboard.
    /// The moves are in simple UCI formats, <from><to>, e.g.: e2e4.
    pub fn feed_moves_to_game_board(&mut self, moves: &Vec<String>) -> Result<(), GameError> {
        if moves.is_empty() {
            return Err(GameError::InvalidMoveFormat);
        }
        let mut color = Color::White;
        for m in moves {
            if m.len() != 4 {
                return Err(GameError::InvalidMoveFormat);
            }
            let from_file = m.chars().nth(0).unwrap() as u8 - b'a';
            let from_rank = m.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;

            let to_file = m.chars().nth(2).unwrap() as u8 - b'a';
            let to_rank = m.chars().nth(3).unwrap().to_digit(10).unwrap() as u8;

            let from = BitBoard::generate_position_index(from_file, from_rank - 1);
            let to = BitBoard::generate_position_index(to_file, to_rank - 1);

            debug!("Processing move: {} from {} to {}", m, from, to);

            let piece = match self.board.get_piece_at_square(from as u32) {
                Some(p) => p,
                None => return Err(GameError::NoPieceAtSquare),
            };
            let piece_type = PieceType::from_char(piece).unwrap();

            if (piece.is_ascii_uppercase() && color == Color::Black)
                || (!piece.is_ascii_uppercase() && color == Color::White)
            {
                return Err(GameError::WrongColorPiece);
            }

            BitBoard::move_piece(&mut self.board, &piece_type, &color, from as u32, to as u32);
            color = if color == Color::White {
                Color::Black
            } else {
                Color::White
            };
        }
        Ok(())
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

    #[test]
    fn test_feed_moves_to_game_board() {
        let mut gs = GameService::new();
        let moves = vec![
            "e2e4".to_string(), // White pawn e2-e4
            "d7d5".to_string(), // Black pawn d7-d5
        ];
        assert!(matches!(gs.feed_moves_to_game_board(&moves), Ok(())));
        assert_eq!(
            gs.board.to_fen(),
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"
        );
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_feed_moves_invalid_file() {
        let mut gs = GameService::new();
        let moves = vec!["92e4".to_string()];
        let _ = gs.feed_moves_to_game_board(&moves);
    }

    #[test]
    fn test_feed_moves_too_short() {
        let mut gs = GameService::new();
        let moves = vec!["e2".to_string()];
        assert!(matches!(
            gs.feed_moves_to_game_board(&moves),
            Err(GameError::InvalidMoveFormat)
        ));
    }

    #[test]
    #[should_panic]
    fn test_feed_moves_invalid_rank() {
        let mut gs = GameService::new();
        let moves = vec!["eae4".to_string()];
        let _ = gs.feed_moves_to_game_board(&moves);
    }

    #[test]
    #[should_panic]
    fn test_feed_moves_out_of_bounds() {
        let mut gs = GameService::new();
        let moves = vec![
            "e9e4".to_string(), // Rank out of bounds
        ];
        let _ = gs.feed_moves_to_game_board(&moves);
    }

    #[test]
    fn test_feed_moves_no_piece() {
        let mut gs = GameService::new();
        let moves = vec![
            "e3e4".to_string(), // No piece at e3
        ];
        assert!(matches!(
            gs.feed_moves_to_game_board(&moves),
            Err(GameError::NoPieceAtSquare)
        ));
    }

    #[test]
    fn test_feed_moves_wrong_color() {
        let mut gs = GameService::new();
        let moves = vec![
            "e7e5".to_string(), // Trying to move black pawn as white
        ];
        assert!(matches!(
            gs.feed_moves_to_game_board(&moves),
            Err(GameError::WrongColorPiece)
        ));
    }
}
