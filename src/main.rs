mod bitboard;
mod game_service;

fn main() {
    println!("Hello, chess enthusiastic!");

    println!("Initializing game...");
    let board = game_service::init_game();
    println!("Game initialized!");

    println!("BitBoard in FEN format: {:?}", board);
}
