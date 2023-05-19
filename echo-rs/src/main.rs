use clap::{Arg, Command};

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
    let text = matches.get_many::<String>("text")
        .unwrap()
        .map(|s| s.as_str().trim())
        .collect::<Vec<_>>()
        .join(" ");

    if omit_newline {
        print!("{}", text);
    } else {
        println!("{}", text);
    }
}
