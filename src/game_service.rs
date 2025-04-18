use crate::bitboard::BitBoard;

pub struct GameService {
    board: BitBoard,
}

impl GameService {
    pub fn new() -> Self {
        GameService {
            board: BitBoard::new(),
        }
    }

    pub fn get_board(&self) -> &BitBoard {
        &self.board
    }

    pub fn set_board(&mut self, board: BitBoard) {
        self.board = board;
    }

    pub fn init_game_from_position(&mut self, fen_str: &str) {
        // TODO: Implement this function to set the board to a specific position
        // based on the FEN string or other representation.
    }

    pub fn reset_game(&mut self) {
        self.board = BitBoard::new();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initial_position_fen() {
        let gs = GameService::new();
        assert_eq!(
            gs.get_board().to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }
}
