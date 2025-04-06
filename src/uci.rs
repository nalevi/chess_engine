enum UciOptionType {
    Check,
    Spin,
    String,
    Button,
    Combo,
}

impl UciOptionType {
    fn from_str(s: &str) -> Option<UciOptionType> {
        match s {
            "check" => Some(UciOptionType::Check),
            "spin" => Some(UciOptionType::Spin),
            "string" => Some(UciOptionType::String),
            "button" => Some(UciOptionType::Button),
            "combo" => Some(UciOptionType::Combo),
            _ => None,
        }
    }

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
    pub is_position: bool,
    pub is_stop: bool,
    pub is_uci: bool,
    pub is_uci_new_game: bool,
    pub is_debug: bool,
    options: Vec<UciOptions>,
    id: UciId,
}

// pub enum EngineToGuiCommand {
//     Id,
//     UciOk,
//     ReadyOk,
//     BestMove",
//     Info = "info",
//     Registration = "registration",
//     Option = "option",
// }

impl Uci {
    pub fn new() -> Self {
        Uci {
            is_ready: false,
            is_quit: false,
            is_go: false,
            is_position: false,
            is_stop: false,
            is_uci: false,
            is_uci_new_game: false,
            is_debug: false,
            options: vec![],
            id: UciId::new(),
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
        self.is_position = false;
        self.is_stop = false;
        self.is_uci = false;
        self.is_uci_new_game = false;
        self.is_debug = false;

        self.options.clear();
        self.init_options();
    }

    pub fn receive(&mut self, command: &str) {
        match command {
            "uci" => self.handle_uci(),
            "isready" => self.is_ready = true,
            "quit" => self.is_quit = true,
            "go" => self.is_go = true,
            "stop" => self.is_stop = true,
            "position" => self.is_position = true,
            "ucinewgame" => self.is_uci_new_game = true,
            "debug" => self.is_debug = true,
            "ponderhit" => {}
            "register" => {}
            "setoption" => {}
            _ => {}
        }
    }

    pub fn send(&self, response: &str) {
        println!("{}", response);
    }

    fn send_options(&self) {
        for option in &self.options {
            self.send(&option.to_string());
        }
    }

    pub fn handle_uci(&mut self) {
        self.is_uci = true;
        self.send(format!("id name {}", self.id.name).as_str());
        self.send(format!("id author {}", self.id.author).as_str());

        self.send_options();

        self.send("uciok");
    }
}
