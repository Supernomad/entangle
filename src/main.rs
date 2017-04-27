extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::process;

mod scanner;

const USAGE: &'static str = "
entangle a high speed tcp port scanner.

Usage:
  entangle (<target>...) [-s <port> | --start-port <port>] [-e <port> | --end-port <port>]
  entangle (-h | --help)
  entangle --version

Options:
  target                         The target ip(s) and/or range(s).
  -s <port> --start-port <port>  Start port for scan. [default: 1]
  -e <port> --end-port <port>    End port for scan. [default: 65535]
  -h --help                      Show this screen.
  --version                      Show version.
";

const VERSION: &'static str = "0.0.1";

const MIN_PORT_NUMBER: i32 = 0;
const MAX_PORT_NUMBER: i32 = 65535;

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_target: Vec<String>,
    flag_start_port: i32,
    flag_end_port: i32,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("entangle v{}", VERSION);
        process::exit(1);
    }

    if args.flag_start_port <= MIN_PORT_NUMBER || args.flag_start_port > MAX_PORT_NUMBER {
        println!("start port must be a valid 0 that is greater than {} and less than {} got: {}", MIN_PORT_NUMBER, MAX_PORT_NUMBER, args.flag_start_port);
        process::exit(1);
    }

    if args.flag_end_port <= MIN_PORT_NUMBER || args.flag_end_port > MAX_PORT_NUMBER {
        println!("end port must be a valid 0 that is greater than {} and less than {} got: {}", MIN_PORT_NUMBER, MAX_PORT_NUMBER, args.flag_end_port);
        process::exit(1);
    }

    for target in args.arg_target.iter() {
        scanner::scan(target, args.flag_start_port, args.flag_end_port);
    }
}
