use clap::Parser;

/// This data structure is used to store the options available in the CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[arg(short, long)]
    pub xpath: String,
    #[arg(last = true, default_value="")]
    pub file_name: String,
    #[arg(skip)]
    pub xml_buffer: Option<String>,
}