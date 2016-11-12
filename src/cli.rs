use std;

fn help(msg: &str) {
    println!("{}", msg);
    println!("NATO WORDS\n\nUsage:\n\nnato -e words to encode\nnato -d words \
              to decode\n");
    std::process::exit(0);
}

pub fn parse() -> (String, Vec<String>) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        help("Error: please enter a command and content\n");
    }
    let ref cmd = args[1];
    if cmd != "-d" && cmd != "-e" {
        help("Error: you have to specify -d or -e\n");
    }
    if args.len() == 2 {
        help("Error: no input to work with\n");
    }
    (cmd.to_string(), args[2..].to_vec())
}
