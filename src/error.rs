use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 输入为空
	#[error("Input is empty")]
	EmptyInput,

	/// 输入文本且给定文件时两者冲突
	#[error("Input hex string and given file have a conflicte")]
	InputAndFileConflict,

	#[error(transparent)]
	IOError(#[from] ::std::io::Error),

	#[error(transparent)]
	HexError(#[from] ::hex::FromHexError),
}

impl Into<clap::error::ErrorKind> for Error {
	fn into(self) -> clap::error::ErrorKind {
		return match self {
			Self::EmptyInput => clap::error::ErrorKind::MissingRequiredArgument,
			Self::InputAndFileConflict => clap::error::ErrorKind::ArgumentConflict,
			Self::IOError(..) => clap::error::ErrorKind::Io,
			Self::HexError(..) => clap::error::ErrorKind::InvalidValue,
		};
	}
}

pub type Result<T> = std::result::Result<T, Error>;
