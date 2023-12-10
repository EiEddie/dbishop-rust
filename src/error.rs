use clap::error as claperr;
use core::fmt;
use hex as hexerr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DBishopError {
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

impl From<hexerr::FromHexError> for DBishopError {
    fn from(value: hexerr::FromHexError) -> Self {
        return match value {
            hexerr::FromHexError::OddLength => DBishopError::OddHexInputLength,
            hexerr::FromHexError::InvalidHexCharacter { c, index } => {
                DBishopError::WrongHexInputChar { c, i: index }
            }
            _ => todo!(),
        };
    }
}

impl Into<claperr::ErrorKind> for DBishopError {
    fn into(self) -> claperr::ErrorKind {
        return match self {
            DBishopError::EmptyInput => claperr::ErrorKind::MissingRequiredArgument,
            DBishopError::OddHexInputLength => claperr::ErrorKind::InvalidValue,
            DBishopError::WrongHexInputChar { .. } => claperr::ErrorKind::InvalidValue,
        };
    }
}
