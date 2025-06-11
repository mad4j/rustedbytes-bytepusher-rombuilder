/// Errori che possono verificarsi durante l'elaborazione
#[derive(Debug)]
pub enum BytePusherError {
    GlobError(glob::GlobError),
    PatternError(glob::PatternError),
    ImageError(image::ImageError),
    IoError(std::io::Error),
    NoFilesFound,
    InvalidFormat,
}

impl std::fmt::Display for BytePusherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytePusherError::GlobError(e) => write!(f, "Glob error: {}", e),
            BytePusherError::PatternError(e) => write!(f, "Pattern error: {}", e),
            BytePusherError::ImageError(e) => write!(f, "Image error: {}", e),
            BytePusherError::IoError(e) => write!(f, "IO error: {}", e),
            BytePusherError::NoFilesFound => write!(f, "No files found matching pattern"),
            BytePusherError::InvalidFormat => write!(f, "Invalid format encountered"),
        }
    }
}

impl std::error::Error for BytePusherError {}

impl From<glob::GlobError> for BytePusherError {
    fn from(error: glob::GlobError) -> Self {
        BytePusherError::GlobError(error)
    }
}

impl From<glob::PatternError> for BytePusherError {
    fn from(error: glob::PatternError) -> Self {
        BytePusherError::PatternError(error)
    }
}

impl From<image::ImageError> for BytePusherError {
    fn from(error: image::ImageError) -> Self {
        BytePusherError::ImageError(error)
    }
}

impl From<std::io::Error> for BytePusherError {
    fn from(error: std::io::Error) -> Self {
        BytePusherError::IoError(error)
    }
}
