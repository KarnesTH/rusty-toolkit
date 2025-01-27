use indicatif::style::TemplateError;
use inquire::InquireError;
use std::fmt;

#[derive(Debug)]
pub enum FileSearchError {
    IoError(std::io::Error),
    InputError(String),
    SearchError(String),
}

impl std::error::Error for FileSearchError {}

impl fmt::Display for FileSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileSearchError::IoError(err) => write!(f, "IO error: {}", err),
            FileSearchError::InputError(msg) => write!(f, "Input error: {}", msg),
            FileSearchError::SearchError(msg) => write!(f, "Search error: {}", msg),
        }
    }
}

impl From<std::io::Error> for FileSearchError {
    fn from(err: std::io::Error) -> Self {
        FileSearchError::IoError(err)
    }
}

impl From<InquireError> for FileSearchError {
    fn from(err: InquireError) -> Self {
        FileSearchError::InputError(err.to_string())
    }
}

impl From<TemplateError> for FileSearchError {
    fn from(err: TemplateError) -> Self {
        FileSearchError::SearchError(err.to_string())
    }
}
