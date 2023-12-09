use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    about = "The hash fingerprint visualization algorithm, like OpenSSH"
)]
struct Cli {
    /// Input data, like a hex string
    data: Option<String>,

    /// Don't echo hex input
    #[arg(short = 'q', long = "quiet", action = clap::ArgAction::SetFalse)]
    is_quiet: bool,

    /// Use file, if '-' use stdin
    #[arg(short = 'i', long = "in")]
    file: Option<String>,

    /// Read the story of Bishop Peter
    #[arg(long = "story", action = clap::ArgAction::SetFalse)]
    is_story: bool,
}

fn main() {
    #[allow(unused_variables)]
    let cli = Cli::parse();
}
