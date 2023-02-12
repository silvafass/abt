#[derive(Debug)]
pub enum BackupError {
    ConfigError(&'static str),
    IOError(std::io::Error),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::ConfigError(field) => {
                write!(f, "{field}")
            }
            BackupError::IOError(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for BackupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            BackupError::ConfigError(_) => None,
            BackupError::IOError(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for BackupError {
    fn from(err: std::io::Error) -> BackupError {
        BackupError::IOError(err)
    }
}

pub type BackupResult<T> = Result<T, BackupError>;
