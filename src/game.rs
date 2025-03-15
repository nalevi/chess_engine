use crate::bitboard::BitBoard;

pub fn init_game() -> BitBoard {
    BitBoard::new(
        0x00FF000000000000, // white pawns   (rank 2)
        0x4200000000000000, // white knights (b1, g1)
        0x2400000000000000, // white bishops (c1, f1)
        0x8100000000000000, // white rooks   (a1, h1)
        0x0800000000000000, // white queen   (d1)
        0x1000000000000000, // white king    (e1)
        0x000000000000FF00, // black pawns   (rank 7)
        0x0000000000000042, // black knights (b8, g8)
        0x0000000000000024, // black bishops (c8, f8)
        0x0000000000000081, // black rooks   (a8, h8)
        0x0000000000000008, // black queen   (d8)
        0x0000000000000010, // black king    (e8)
    )
}
