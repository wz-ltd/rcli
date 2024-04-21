use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "0.1.0", author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other format")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, default_value = "assets/juventus.csv", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file_exists(path: &str) -> Result<String, &'static str> {
    if Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err("file not exists")
    }
}
