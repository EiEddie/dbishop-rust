use clap::{CommandFactory, Parser};
use dbishop::error::*;
use dbishop::gen;

#[derive(Parser, Debug)]
#[command(version,
          about = "The hash fingerprint visualization algorithm, like OpenSSH")]
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

fn run() -> Result<()> {
	let cli = Cli::parse();

	#[cfg(debug_assertions)]
	dbg!(&cli);

	if cli.is_story {
		todo!();
	}

	return match (cli.data, cli.file) {
		// 传入参数为空
		(None, None) => Err(Error::EmptyInput),

		// 仅传入文件, 输出文件指纹
		(None, Some(file)) => {
			let fp = gen::fp_of_file(&file)?;
			if !cli.is_quiet {
				println!("fingerprint of file `{}`:", file);
			}
			print!("{}", fp);
			Ok(())
		},

		// 仅传入十六进制字符串, 输出字符串指纹
		(Some(data), None) => {
			let fp = gen::fp_of_str(&data)?;
			if !cli.is_quiet {
				println!("fingerprint of hex `{}`:", data);
			}
			print!("{}", fp);
			Ok(())
		},

		// 同时传入两者, 参数矛盾
		(Some(_), Some(_)) => Err(Error::InputAndFileConflict),
	};
}

fn main() {
	let mut cmd = Cli::command();
	if let Err(e) = run() {
		let msg = e.to_string();
		cmd.error(e.into(), msg).exit();
	}
}
