use clap::Parser;

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