use std::fmt;

#[derive(Debug)]
pub enum Error {
    Read,
    Parse,
    Server,
    Connection,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => {
                write!(
                    f,
                    "Could not read the endpoints .toml file, make sure it exists."
                )
            }
            Self::Parse => {
                write!(
                    f,
                    "Could not parse the endpoints .toml file, please check for syntax errors."
                )
            }
            Self::Server => {
                write!(f, "Server error.")
            }
            Self::Connection => {
                write!(f, "Failed to connect to caddy.")
            }
        }
    }
}
