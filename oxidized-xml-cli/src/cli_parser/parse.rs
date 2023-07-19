use std::{env, fs};
use std::io::{BufRead, Read, stdin, StdinLock};
use crate::Opts;

pub fn parse_cli() -> Opts {
    let mut options = parse_arguments();
    // TODO: Handle piped-in input
    // if options.xml_buffer.is_empty() {
    //     options.xml_buffer = parse_stdin()
    // }
    options
}

fn parse_arguments() -> Opts {
    // let stdin = stdin().lock();
    let mut options = Opts::new();
    // options.stdin_handle = stdin;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--xpath" => parse_xpath(&mut options, args.next()),
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0)
            },
            "--" => parse_input_to_buffer(&mut options, args.next()),
            _ => { parse_input_to_buffer(&mut options, Some(arg))}
        }
    }
    options
}

fn parse_xpath(opts: &mut Opts, arg: Option<String>) {
    match arg {
        Some(xpath) => opts.xpath = xpath,
        _ => {panic!("Invalid XPath expression.")}
    }
}

fn print_usage() {
    println!("Usage : ")
}

fn parse_input_to_buffer(options: &mut Opts, arg: Option<String>) {
    options.xml_buffer = match arg {
        Some(file_name) => parse_input_file(file_name),
        None => String::new()
    };
}

fn parse_input_file(file_name: String) -> String{
    println!("filename: {}", file_name);
    match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(error) => panic!("{}", error.to_string())
    }
}

fn parse_stdin() -> String {
    let mut buffer = String::new();
    let mut line = String::new();
    let stdin_handle = stdin();
    loop {
        match stdin_handle.read_line(&mut line) {
            Ok(
                length) => {
                if length == 0 || line.eq("\n") {
                    println!("lenght is empty");
                    break;
                }
                println!("{}", line);
                println!("length: {}", length);
                buffer.push_str(line.as_str());
                line.clear()
            },
            Err(_) => {break}
        }
    };
    buffer
}

