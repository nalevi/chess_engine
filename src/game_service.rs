use crate::bitboard::Color;

use crate::bitboard::BitBoard;

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

    pub fn feed_moves_to_game(&mut self, moves: Vec<String>) {
        let mut color = Color::White;
        for m in moves {

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
