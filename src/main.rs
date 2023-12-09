use clap::Parser;
use std::{
    fmt,
    ops::{Add, AddAssign},
};

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

#[derive(Clone, Copy)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        return Pos(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

struct Field {
    width: usize,
    height: usize,
    val: Vec<i32>,
    move_pnt: Pos,
}

impl Field {
    fn new(width: usize, height: usize) -> Field {
        return Field {
            width,
            height,
            val: vec![0; width * height],
            move_pnt: Pos(width as i32 / 2, height as i32 / 2),
        };
    }

    /// 将索引转为 `field` 中的位置
    fn index_to_pos(&self, index: usize) -> Pos {
        return Pos((index % self.width) as i32, (index / self.width) as i32);
    }

    /// 将 `field` 中的位置转为索引
    fn pos_to_index(&self, pos: Pos) -> usize {
        let w = self.width as i32;
        let h = self.height as i32;
        return ((pos.0 + pos.1 * w) % (w * h)) as usize;
    }

    /// 根据传入的方向移动两次
    /// 仅 [`u8`] 的后 4 bit 为有效方向
    /// - `00` LU
    /// - `01` RU
    /// - `10` LD
    /// - `11` RD
    fn pnt_move(&mut self, dirs: u8) -> Pos {
        let mut dirs = dirs;
        for _ in 0..2 {
            self.move_pnt += match dirs & 0b11 {
                0b00 => Pos(-1, -1),
                0b01 => Pos(1, -1),
                0b10 => Pos(-1, 1),
                0b11 => Pos(1, 1),
                _ => Pos(0, 0),
            };
            dirs >>= 2;
        }

        let w = self.width as i32;
        let h = self.height as i32;
        self.move_pnt.0 = self.move_pnt.0 % w + w;
        self.move_pnt.1 = self.move_pnt.1 % h + h;
        return self.move_pnt;
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
