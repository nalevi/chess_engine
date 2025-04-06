use chess_engine::uci::Uci;
use std::io::Write;

// Helper function to capture stdout
fn capture_stdout<F>(f: F) -> Vec<String>
where
    F: FnOnce(),
{
    use std::io::BufRead;
    let mut stdout_lines = Vec::new();

    let stdout = std::io::stdout();
    let output = std::io::Cursor::new(Vec::new());
    {
        let mut handle = stdout.lock();
        f();
        handle.flush().unwrap();
    }

    let reader = std::io::BufReader::new(output);
    for line in reader.lines() {
        stdout_lines.push(line.unwrap());
    }

    stdout_lines
}

#[test]
fn test_given_uci_protocol_when_uci_then_id_options_uciok() {
    let mut uci = Uci::new();

    let output = capture_stdout(|| {
        uci.start();
        uci.receive("uci");
    });

    let expected_output = vec![
        "id name Srut",
        "id author Levente Nagy",
        "option name Hash type spin default 16 min 1 max 1024",
        "option name Threads type spin default 1 min 1 max 64",
        "option name Ponder type check default false",
        "option name OwnBook type check default false",
        "option name UCI_ShowCurrLine type check default false",
        "option name UCI_ShowRefutations type check default false",
        "option name UCI_LimitStrength type check default false",
        "option name UCI_AnalyseMode type check default false",
        "option name UCI_EngineAbout type string default Srut by Levente Nagy",
        "option name UCI_SetPositionValue type string default ",
        "uciok",
    ];

    assert!(uci.is_uci);
    for (expected, actual) in expected_output.iter().zip(output.iter()) {
        assert_eq!(expected, actual);
    }
}
