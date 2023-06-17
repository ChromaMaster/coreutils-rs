#[derive(Debug)]
pub struct Config {
    pub omit_newline: bool,
}

pub fn run(config: Config, input: Vec<&String>) {

    let text = input
        .iter()
        .map(|s| s.as_str().trim())
        .collect::<Vec<_>>()
        .join(" ");

    if config.omit_newline {
        print!("{}", text);
    } else {
        println!("{}", text);
    }
}