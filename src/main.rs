use chess_engine::uci;

fn main() {
    println!("Hello, chess enthusiastic!");

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
