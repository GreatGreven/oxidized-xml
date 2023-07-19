mod cli_parser;

use cli_parser::{options::*, parse::*};

fn main() {
    let options: Opts = parse_cli();
    let values = oxidized_xml_fetcher::retrieve_values_in_xml_by(
        options.xpath.as_str(),
        options.xml_buffer.expect("No Xml-structure to parse").as_str()
    ).expect("Could not retrieve value in given xml");
    print_values(values);
}

fn print_values(values: Vec<String>) {
    for value in values {
        println!("{}", value)
    }
}


