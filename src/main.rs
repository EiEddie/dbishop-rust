use clap::{CommandFactory, Parser, Subcommand};
use dbishop::error::*;
use dbishop::gen;

#[derive(Parser, Debug)]
#[command(version,
          about = "The hash fingerprint visualization algorithm, like OpenSSH")]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,

	/// Don't echo hex input
	#[arg(short = 'q', long = "quiet", action = clap::ArgAction::SetTrue)]
	is_quiet: bool,

	/// Read the story of Bishop Peter
	#[arg(long = "story", action = clap::ArgAction::SetTrue)]
	is_story: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// Fingerprint of hex string
	Str { data: String },

	/// Fingerprint of a byte array
	Byte {
		#[arg(short = 'i', long = "in")]
		file: String,
	},

	/// Fingerprint of a file, use md5
	File {
		#[arg(short = 'f', long = "file")]
		file: String,
	},
}

fn run() -> Result<()> {
	let cli = Cli::parse();

	#[cfg(debug_assertions)]
	dbg!(&cli);

	if cli.is_story {
		print!(
		       "Bishop Peter finds himself in the middle of an\nambient atrium. There are walls \
		        on all four sides\nand apparently there is no exit. The floor is paved\nwith \
		        square tiles, strictly alternating between\nblack and white. His head heavily \
		        aching—probably\nfrom too much wine he had before—he starts\nwandering around \
		        randomly. Well, to be exact, he\nonly makes diagonal steps—just like a bishop on \
		        a\nchess board. When he hits a wall, he moves to the\nside, which takes him from \
		        the black tiles to the\nwhite tiles (or vice versa). And after each move,\nhe \
		        places a coin on the floor, to remember that he\nhas been there before. After 64 \
		        steps, just when no\ncoins are left, Peter suddenly wakes up. What a\nstrange \
		        dream!\n"
		);
		return Ok(());
	}

	let fp: String = match cli.command {
		Commands::Str { data } => {
			let fp = gen::fp_of_str(&data)?;
			if !cli.is_quiet {
				println!("fingerprint of str `{}`:", data);
			}
			fp
		},
		Commands::Byte { file } => {
			let fp = gen::fp_of_byte_on_file(&file)?;
			if !cli.is_quiet {
				println!("fingerprint of bytes on file `{}`:", file);
			}
			fp
		},
		Commands::File { file } => {
			let fp = gen::fp_of_file_by_sha256(&file)?;
			if !cli.is_quiet {
				println!("fingerprint of sha256 on file `{}`:", file);
			}
			fp
		},
	};
	print!("{}", fp);
	Ok(())
}

fn main() {
	let mut cmd = Cli::command();
	if let Err(e) = run() {
		let msg = e.to_string();
		cmd.error(e.into(), msg).exit();
	}
}
