use std::cell::BorrowError;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    GetSettingsError,
    ReadFileError,
}

impl From<BorrowError> for Error {
    fn from(_be: BorrowError) -> Error {
        Error::GetSettingsError
    }
}

impl From<std::io::Error> for Error {
    fn from(_ie: std::io::Error) -> Error {
        Error::ReadFileError
    }
}

impl Error {
    pub fn explain(&self) -> String {
        match self {
            Error::GetSettingsError => "failed to get settings.".to_owned(),
            Error::ReadFileError => "failed to read files.".to_owned(),
        }
    }
}
