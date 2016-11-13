mod encode;
mod decode;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
NATO words.

Usage:
  nato encode [options] <text>...
  nato decode [options] <text>...
  nato help
  nato version

Options:
  -y, --yell    YELL the results.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_text: Option<Vec<String>>,
    cmd_encode: bool,
    cmd_decode: bool,
    cmd_help: bool,
    cmd_version: bool,
    flag_yell: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if let Some(words) = args.arg_text {
        if args.cmd_encode {
            println!("{}", encode::translated(words, args.flag_yell))
        } else if args.cmd_decode {
            println!("{}", decode::translated(words, args.flag_yell))
        }
    }
}