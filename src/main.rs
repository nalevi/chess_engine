use chess_engine::uci;
use log::info;

fn main() {
    env_logger::Builder::from_env("RUST_LOG").init();
    info!("Hello, chess enthusiastic!");

    let mut uci = uci::Uci::new();
    uci.start();

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        uci.receive(input);

        if input == "quit" {
            break;
        }
    }
}
