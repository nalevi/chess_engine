#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

pub struct BitBoard {
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
}

impl BitBoard {
    pub fn new(
        wp: u64,
        wn: u64,
        wb: u64,
        wr: u64,
        wq: u64,
        wk: u64,
        bp: u64,
        bn: u64,
        bb: u64,
        br: u64,
        bq: u64,
        bk: u64,
    ) -> Self {
        BitBoard {
            white_pawns: wp,
            white_knights: wn,
            white_bishops: wb,
            white_rooks: wr,
            white_queen: wq,
            white_king: wk,
            black_pawns: bp,
            black_knights: bn,
            black_bishops: bb,
            black_rooks: br,
            black_queen: bq,
            black_king: bk,
        }
    }

    pub fn get_pawns(&self, color: Color) -> u64 {
        match color {
            Color::White => self.white_pawns,
            Color::Black => self.black_pawns,
        }
    }
}
