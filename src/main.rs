mod bitboard;
mod game;

fn main() {
    println!("Hello, chess enthusiastic!");

    println!("Initializing game...");
    let board = game::init_game();
    println!("Game initialized!");

    println!(
        "White pawns: {:016X}",
        board.get_pawns(bitboard::Color::White)
    );
}
