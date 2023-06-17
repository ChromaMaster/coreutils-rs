use clap::{Arg, Command};
use echo_rs::Config;

fn cli() -> Command {
    Command::new("My super command")
        .version("0.1.0")
        .about("Echo written in Rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Text to echo")
                .required(true)
                .num_args(1..)
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not output the trailing newline")
                .num_args(0)
        )
}

fn main() {
    let matches = cli().get_matches();

    let omit_newline = matches.get_flag("omit_newline");
    let input = matches.get_many::<String>("text").unwrap().collect::<Vec<&String>>();

    echo_rs::run(Config { omit_newline }, input)
}
