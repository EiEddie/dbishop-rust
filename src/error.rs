use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 输入为空
	#[error("Input is empty")]
	EmptyInput,

	#[error(transparent)]
	HexError(#[from] ::hex::FromHexError),
}

impl Into<clap::error::ErrorKind> for Error {
	fn into(self) -> clap::error::ErrorKind {
		return match self {
			Self::EmptyInput => clap::error::ErrorKind::MissingRequiredArgument,
			Self::HexError(..) => clap::error::ErrorKind::InvalidValue,
		};
	}
}

pub type Result<T> = std::result::Result<T, Error>;
