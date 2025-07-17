use crate::game_service::{GameService, SearchResult};

use log::{debug, error};
use tokio::runtime::Runtime;

#[derive(Debug, PartialEq)]
enum UciOptionType {
    Check,
    Spin,
    String,
    Button,
    Combo,
}

impl UciOptionType {
    fn to_str(&self) -> &str {
        match self {
            UciOptionType::Check => "check",
            UciOptionType::Spin => "spin",
            UciOptionType::String => "string",
            UciOptionType::Button => "button",
            UciOptionType::Combo => "combo",
        }
    }
}

struct UciOptions {
    pub name: String,
    pub default: String,
    pub value: String,
    pub type_: UciOptionType,
    pub min: String,
    pub max: String,
    pub var: Vec<String>,
}

impl UciOptions {
    pub fn new(name: &str) -> Self {
        UciOptions {
            name: name.to_owned(),
            default: String::new(),
            value: String::new(),
            type_: UciOptionType::String,
            min: String::new(),
            max: String::new(),
            var: Vec::new(),
        }
    }

    pub fn default_value(mut self, value: &str) -> Self {
        self.default = value.to_owned();
        self.value = value.to_owned();
        self
    }

    pub fn option_type(mut self, type_: UciOptionType) -> Self {
        self.type_ = type_;
        self
    }

    pub fn min(mut self, min: &str) -> Self {
        self.min = min.to_owned();
        self
    }

    pub fn max(mut self, max: &str) -> Self {
        self.max = max.to_owned();
        self
    }

    pub fn var(mut self, var: &str) -> Self {
        self.var.push(var.to_owned());
        self
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("option name {} type {}", self.name, self.type_.to_str());
        if !self.default.is_empty() {
            result.push_str(&format!(" default {}", self.default));
        }
        // if !self.value.is_empty() {
        //     result.push_str(&format!(" value {}", self.value));
        // }
        if !self.min.is_empty() {
            result.push_str(&format!(" min {}", self.min));
        }
        if !self.max.is_empty() {
            result.push_str(&format!(" max {}", self.max));
        }
        if !self.var.is_empty() {
            result.push_str(&format!(" var {}", self.var.join(" var ")));
        }
        result
    }
}

struct UciId {
    pub name: String,
    pub author: String,
}

impl UciId {
    pub fn new() -> Self {
        UciId {
            name: "Srut".to_owned(),
            author: "Levente Nagy".to_owned(),
        }
    }
}

pub struct Uci {
    pub is_ready: bool,
    pub is_quit: bool,
    pub is_go: bool,
    pub is_uci: bool,
    pub is_debug: bool,
    options: Vec<UciOptions>,
    id: UciId,
    game_service: GameService,
    runtime: Runtime,
    log_path: String,
}

impl Uci {
    pub fn new(log_file: &str) -> Self {
        Uci {
            is_ready: false,
            is_quit: false,
            is_go: false,
            is_uci: false,
            is_debug: false,
            options: vec![],
            id: UciId::new(),
            game_service: GameService::new(1),
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
            log_path: log_file.to_owned(),
        }
    }

    pub fn start(&mut self) {
        self.reset();
    }

    pub fn init_options(&mut self) {
        self.options.push(
            UciOptions::new("Hash")
                .default_value("16")
                .option_type(UciOptionType::Spin)
                .min("1")
                .max("1024"),
        );
        self.options.push(
            UciOptions::new("Threads")
                .default_value("1")
                .option_type(UciOptionType::Spin)
                .min("1")
                .max("64"),
        );
        self.options.push(
            UciOptions::new("Ponder")
                .default_value("false")
                .option_type(UciOptionType::Check),
        );
        self.options.push(
            UciOptions::new("OwnBook")
                .default_value("false")
                .option_type(UciOptionType::Check),
        );
        self.options.push(
            UciOptions::new("UCI_ShowCurrLine")
                .default_value("false")
                .option_type(UciOptionType::Check),
        );
        self.options.push(
            UciOptions::new("UCI_ShowRefutations")
                .default_value("false")
                .option_type(UciOptionType::Check),
        );
        self.options.push(
            UciOptions::new("UCI_LimitStrength")
                .default_value("false")
                .option_type(UciOptionType::Check),
        );
        self.options.push(
            UciOptions::new("UCI_AnalyseMode")
                .default_value("false")
                .option_type(UciOptionType::Check),
        );
        self.options.push(
            UciOptions::new("UCI_EngineAbout")
                .default_value("Srut by Levente Nagy")
                .option_type(UciOptionType::String),
        );
        self.options.push(
            UciOptions::new("UCI_SetPositionValue")
                .default_value("")
                .option_type(UciOptionType::String),
        );

        self.options.push(
            UciOptions::new("Log file")
                .default_value(self.log_path.as_str())
                .option_type(UciOptionType::String),
        );
    }

    pub fn reset(&mut self) {
        self.is_ready = false;
        self.is_quit = false;
        self.is_go = false;
        self.is_uci = false;
        self.is_debug = false;

        self.options.clear();
        self.init_options();
    }

    pub fn receive(&mut self, command: &str) {
        debug!("Received command: {}", command);
        if command.is_empty() {
            return;
        }
        if self.is_debug {
            Self::send_info(&format!("string {}", command));
        }
        match command {
            "uci" => self.handle_uci(),
            "isready" => self.handle_isready(),
            "quit" => self.handle_quit(),
            s if s.contains("go") => self.handle_go(s),
            "stop" => self.handle_stop(),
            s if s.contains("position") => self.handle_position(s),
            "ucinewgame" => self.handle_newgame(),
            "debug on" => self.is_debug = true,
            "debug off" => self.is_debug = false,
            //"ponderhit" => {}
            s if s.contains("register") => self.handle_register(s),
            s if s.contains("setoption") => self.handle_setoption(s),
            _ => {}
        }
    }

    pub fn send(response: &str) {
        println!("{}", response);
    }

    pub fn send_info(info_string: &str) {
        let info = format!("info {}", info_string);
        Self::send(&info);
    }

    fn send_options(&self) {
        for option in &self.options {
            Self::send(&option.to_string());
        }
    }

    fn handle_quit(&mut self) {
        self.is_quit = true;
        // TODO: Handle quit command
    }

    pub fn handle_uci(&mut self) {
        self.is_uci = true;
        Self::send(format!("id name {}", self.id.name).as_str());
        Self::send(format!("id author {}", self.id.author).as_str());

        self.send_options();

        Self::send("uciok");
        self.is_ready = true;
    }

    // handles the setoption command: setoption name <name> value <value>
    fn handle_setoption(&mut self, set_str: &str) {
        let parts: Vec<&str> = set_str.split_whitespace().collect();
        if parts.len() < 5 {
            return;
        }

        let name = parts[2];
        let value = parts[4];

        for option in &mut self.options {
            if option.name.to_lowercase() == name.to_lowercase() {
                if option.type_ == UciOptionType::Button {
                    // TODO: handle button type
                } else {
                    option.value = value.to_owned();
                    break;
                }
            }
        }
    }

    fn handle_isready(&mut self) {
        if self.is_ready {
            Self::send("readyok");
        }
    }

    fn handle_register(&mut self, cmd_str: &str) {
        if cmd_str.contains("later") {
            return;
        }

        let mut iter = cmd_str.split_whitespace().skip(1);

        let mut name_val: String = String::new();
        let mut code_val: String = String::new();
        let val = iter.next();
        if val == Some("name") {
            while let Some(name) = iter.next() {
                if name == "code" {
                    name_val = name_val.trim().to_string();

                    code_val = iter.next().unwrap_or("").to_string();
                    break;
                }
                name_val += name;
                name_val += " ";
            }
        }

        // TODO: do something with the name and code
        Self::send_info(&format!("string Name: {} Code: {}", name_val, code_val));
    }

    fn handle_newgame(&mut self) {
        self.is_ready = false;
        self.game_service.reset_game();
        self.game_service.set_num_threads(
            self.options
                .iter()
                .find(|o| o.name == "Threads")
                .map_or(1, |o| o.value.parse().unwrap_or(1)),
        );
        self.is_ready = true;
    }

    fn handle_position(&mut self, cmd_str: &str) {
        self.is_ready = false;
        let mut iter = cmd_str.split_whitespace().skip(1);
        let token = iter.next();

        if token == Some("startpos") {
            self.game_service.reset_game();
            debug!("Table reset!");
        } else if token == Some("fen") {
            let fen_parts: Vec<&str> = iter
                .by_ref()
                .take_while(|&token| token != "moves")
                .collect();
            let fen_str = fen_parts.join(" ");
            self.game_service.init_game_from_position(&fen_str);
        }
        // skip the "moves" token
        //let _ = iter.next();
        let mut moves = Vec::new();
        for move_str in iter {
            debug!("Received move: {}", move_str);
            moves.push(move_str.to_string());
        }

        debug!("Received moves: {:?}", moves);

        match self.game_service.feed_moves_to_game_board(&moves) {
            Ok(_) => {
                debug!("Moves executed successfully");
            }
            Err(e) => {
                error!("Error during moves execution: {:?}", e);
            }
        }

        self.is_ready = true;
    }

    fn handle_go(&mut self, cmd_str: &str) {
        self.is_go = true;

        let mut iter = cmd_str.split_whitespace().skip(1);
        let token = iter.next();
        let mut is_infinite = false;

        let mut search_res = SearchResult {
            depth: 0,
            score: 0,
            best_move: String::new(),
        };

        let info_sender = move |result: SearchResult| {
            let info = format!(
                "score cp {} depth {} pv {}",
                result.score, result.depth, result.best_move
            );

            Self::send_info(&info);
        };

        if token == Some("infinite") {
            // Handle infinite go
            is_infinite = true;
            debug!("go command received with infinite time");

            // TODO: somehow watch the state of stop and go and call stop_search() on stop

            let res = self
                .runtime
                .block_on(self.game_service.search_moves(info_sender, 1));

            match res {
                Ok(result) => {
                    search_res = result;
                }
                Err(e) => {
                    error!("Error during search: {:?}", e);
                }
            }
        } else {
            // Handle other go commands
            debug!("go command received: {}", cmd_str);
            let mut depth = 0;
            let mut time = 0;
            let mut moves_to_search = 0;
            let mut nodes = 0;

            while let Some(arg) = iter.next() {
                match arg {
                    "depth" => {
                        depth = iter.next().and_then(|d| d.parse().ok()).unwrap_or(0);
                    }
                    "movetime" => {
                        time = iter.next().and_then(|t| t.parse().ok()).unwrap_or(0);
                    }
                    "nodes" => {
                        nodes = iter.next().and_then(|n| n.parse().ok()).unwrap_or(0);
                    }
                    "movestogo" => {
                        moves_to_search = iter.next().and_then(|m| m.parse().ok()).unwrap_or(0);
                    }
                    _ => {}
                }
            }
            if depth > 0 {
                debug!("Searching with depth: {}", depth);
            } else if time > 0 {
                debug!("Searching with time: {} ms", time);
            } else if nodes > 0 {
                debug!("Searching with nodes: {}", nodes);
            } else if moves_to_search > 0 {
                debug!("Searching with moves to search: {}", moves_to_search);
            } else {
                debug!("Searching with default parameters");
            }

            let res = self
                .runtime
                .block_on(self.game_service.search_moves(info_sender, depth));

            match res {
                Ok(result) => {
                    search_res = result;
                }
                Err(e) => {
                    error!("Error during search: {:?}", e);
                }
            }
        }

        Self::send(format!("bestmove {}", search_res.best_move).as_str());
    }

    pub fn handle_stop(&mut self) {
        self.is_go = false;
    }
}
