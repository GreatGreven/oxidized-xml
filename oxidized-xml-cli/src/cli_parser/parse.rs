use std::fs;
use std::io::stdin;
use atty::Stream;
use crate::Opts;
use clap::Parser;

pub fn parse_cli() -> Opts {
    let mut options = parse_arguments();
    if is_piped_stdin(&options.xml_buffer) {
        options.xml_buffer = Some(parse_stdin())
    }
    options
}

fn is_piped_stdin(buffer: &Option<String>) -> bool{
    atty::isnt(Stream::Stdin) && buffer.is_none()
}

fn parse_arguments() -> Opts {
    let mut options = Opts::parse();
    options.xml_buffer = parse_input_file_to_buffer(&options.file_name);
    options
}


fn parse_input_file_to_buffer(file_name: &String) -> Option<String> {
    if file_name.is_empty() && is_piped_stdin(&None) {
        return None
    }
    let mut text = String::new();
    match fs::read_to_string(file_name) {
        Ok(contents) => text = contents,
        Err(error) => {
            eprintln!("{}", error);

        }
    };
    Some(text)
}

fn parse_stdin() -> String {
    let mut buffer = String::new();
    let mut line = String::new();
    let stdin_handle = stdin();

    loop {
        match stdin_handle.read_line(&mut line) {
            Ok(length) => {
                if length == 0 || line.eq("\n") {
                    break;
                }
                buffer.push_str(line.as_str());
                line.clear()
            },
            Err(_) => { break }
        }
    }
    buffer
}

