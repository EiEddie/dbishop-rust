use clap::{CommandFactory, Parser};
use dbishop as db;
use hex;

mod error;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "The hash fingerprint visualization algorithm, like OpenSSH"
)]
struct Cli {
    /// Input data, like a hex string
    data: Option<String>,

    /// Don't echo hex input
    #[arg(short = 'q', long = "quiet", action = clap::ArgAction::SetTrue)]
    is_quiet: bool,

    /// Use file, if '-' use stdin
    #[arg(short = 'i', long = "in")]
    file: Option<String>,

    /// Read the story of Bishop Peter
    #[arg(long = "story", action = clap::ArgAction::SetTrue)]
    is_story: bool,
}

fn cli() -> Result<(), error::DBishopError> {
    let cli = Cli::parse();

    #[cfg(debug_assertions)]
    dbg!(&cli);

    if cli.is_story {
        todo!();
    }

    if cli.data == None && cli.file == None {
        return Err(error::DBishopError::EmptyInput);
    }

    let seq = hex::decode(cli.data.as_ref().unwrap())?;
    if !cli.is_quiet {
        println!("fingerprint of `{}`:", cli.data.as_ref().unwrap());
    }
    print!("{}", db::fingerprint(seq));

    return Ok(());
}

fn main() {
    let mut cmd = Cli::command();
    if let Err(e) = cli() {
        cmd.error(e.into(), e.to_string()).exit();
    }
}
