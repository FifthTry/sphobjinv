#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IoError: {source}")]
    Io {
        #[from]
        source: std::io::Error,
        // backtrace: std::backtrace::Backtrace,
    },
    #[error("InvalidHeader: {message}")]
    InvalidHeader { message: String },
}
