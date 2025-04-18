use crate::game_service::GameService;

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
    pub is_stop: bool,
    pub is_uci: bool,
    pub is_debug: bool,
    options: Vec<UciOptions>,
    id: UciId,
    game_service: GameService,
}

impl Uci {
    pub fn new() -> Self {
        Uci {
            is_ready: false,
            is_quit: false,
            is_go: false,
            is_stop: false,
            is_uci: false,
            is_debug: false,
            options: vec![],
            id: UciId::new(),
            game_service: GameService::new(),
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
    }

    pub fn reset(&mut self) {
        self.is_ready = false;
        self.is_quit = false;
        self.is_go = false;
        self.is_stop = false;
        self.is_uci = false;
        self.is_debug = false;

        self.options.clear();
        self.init_options();
        self.game_service.reset_game();
    }

    pub fn receive(&mut self, command: &str) {
        if self.is_debug {
            self.send_info(&format!("string {}", command));
        }
        match command {
            "uci" => self.handle_uci(),
            "isready" => self.handle_isready(),
            "quit" => self.handle_quit(),
            //"go" => self.is_go = true,
            //"stop" => self.is_stop = true,
            "position" => self.handle_position(command),
            "ucinewgame" => self.handle_newgame(),
            "debug on" => self.is_debug = true,
            "debug off" => self.is_debug = false,
            //"ponderhit" => {}
            s if s.contains("register") => self.handle_register(s),
            s if s.contains("setoption") => self.handle_setoption(s),
            _ => {}
        }
    }

    pub fn send(&self, response: &str) {
        println!("{}", response);
    }

    pub fn send_info(&self, info_string: &str) {
        let info = format!("info {}", info_string);
        self.send(&info);
    }

    fn send_options(&self) {
        for option in &self.options {
            self.send(&option.to_string());
        }
    }

    fn handle_quit(&mut self) {
        self.is_quit = true;
        // TODO: Handle quit command
    }

    pub fn handle_uci(&mut self) {
        self.is_uci = true;
        self.send(format!("id name {}", self.id.name).as_str());
        self.send(format!("id author {}", self.id.author).as_str());

        self.send_options();

        self.send("uciok");
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
            self.send("readyok");
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
        println!("Name: {}", name_val);
        println!("Code: {}", code_val);
    }

    fn handle_newgame(&mut self) {
        self.is_ready = false;
        self.game_service.reset_game();
        self.is_ready = true;
    }

    fn handle_position(&mut self, cmd_str: &str) {
        self.is_ready = false;

        let mut iter = cmd_str.split_whitespace().skip(1);
        let token = iter.next();

        if token == Some("startpos") {
            self.game_service.reset_game();
        } else if token == Some("fen") {
            self.game_service.init_game_from_position(token.unwrap());
        }

        // skip the "moves" token
        let _ = iter.next();
        let mut moves = Vec::new();
        for move_str in iter {
            moves.push(move_str.to_string());
        }

        // TODO: execute moves on the board

        self.is_ready = true;
    }
}
