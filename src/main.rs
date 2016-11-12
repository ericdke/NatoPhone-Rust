mod cli;
mod encode;
mod decode;

fn main() {
    let (cmd, words) = cli::parse();

    if cmd == "-e" {
        println!("{}", encode::translated(words));
    } else {
        println!("{}", decode::translated(words));
    }
}