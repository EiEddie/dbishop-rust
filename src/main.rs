use clap::Parser;
use std::fmt;

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

struct Pos(i32, i32);

struct Field {
    width: usize,
    height: usize,
    val: Vec<i32>,
}

impl Field {
    fn new(width: usize, height: usize) -> Field {
        return Field {
            width,
            height,
            val: vec![0; width * height],
        };
    }

    fn index_to_pos(&self, index: usize) -> Pos {
        return Pos((index % self.width) as i32, (index / self.width) as i32);
    }

    fn pos_to_index(&self, pos: Pos) -> usize {
        let w = self.width as i32;
        let h = self.height as i32;
        return ((pos.0 + pos.1 * w) % (w * h)) as usize;
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chars_list = [
            ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^', 'S', 'E',
        ];

        write!(f, "+{}+\n", "-".repeat(self.width))?;
        for y in 0..self.height {
            write!(f, "|")?;
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    chars_list[self.val[self.pos_to_index(Pos(x as i32, y as i32))] as usize]
                )?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "+{}+\n", "-".repeat(self.width))?;
        return Ok(());
    }
}

fn main() {
    #[allow(unused_variables)]
    let cli = Cli::parse();
}
