use chess_engine::uci;
use env_logger::{Builder, Env};
use log::info;
use std::fs::File;

fn main() {
    let abs_path = std::env::current_exe().unwrap();
    let log_file = File::create("srut.log").unwrap();
    Builder::from_env(Env::default().default_filter_or("debug"))
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
    info!("Hello, chess enthusiastic!");

    let log_path = abs_path.parent().unwrap().join("srut.log");
    let mut uci = uci::Uci::new(log_path.to_str().unwrap());
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
