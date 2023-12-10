use clap::{error::ErrorKind, Parser};
use core::fmt;
use hex;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DBishopError {
    /// 输入为空
    EmptyInput,

    /// 输入的十六进制字符串始终应为偶数
    OddHexInputLength,

    /// 输入的十六进制字符串存在非法字符
    WrongHexInputChar { c: char, i: usize },
}

#[cfg(feature = "std")]
impl std::error::Error for DBishopError {}

impl fmt::Display for DBishopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match *self {
            Self::EmptyInput => write!(f, "Input is empty."),
            Self::OddHexInputLength => write!(f, "The number of digits is not even."),
            Self::WrongHexInputChar { c, i } => {
                write!(f, "Input string has a worng char `{}` in {}.", c, i)
            }
        };
    }
}

impl From<hex::FromHexError> for DBishopError {
    fn from(value: hex::FromHexError) -> Self {
        return match value {
            hex::FromHexError::OddLength => DBishopError::OddHexInputLength,
            hex::FromHexError::InvalidHexCharacter { c, index } => {
                DBishopError::WrongHexInputChar { c, i: index }
            }
            hex::FromHexError::InvalidStringLength => todo!(),
        };
    }
}

impl Into<ErrorKind> for DBishopError {
    fn into(self) -> ErrorKind {
        return match self {
            DBishopError::EmptyInput => ErrorKind::MissingRequiredArgument,
            DBishopError::OddHexInputLength => ErrorKind::InvalidValue,
            DBishopError::WrongHexInputChar { .. } => ErrorKind::InvalidValue,
        };
    }
}

#[derive(Parser)]
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

fn main() {
    #[allow(unused_variables)]
    let cli = Cli::parse();
}
