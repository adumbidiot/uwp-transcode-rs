pub use crate::bindings::windows::media::transcoding::TranscodeFailureReason;

pub type TranscodeResult<T> = Result<T, TranscodeError>;

#[derive(Debug)]
pub enum TranscodeError {
    WinRt(winrt::Error),
    Io(std::io::Error),
    NonUtf8Path,

    PrepareTranscodeFailure(TranscodeFailureReason),
}

impl From<winrt::Error> for TranscodeError {
    fn from(e: winrt::Error) -> Self {
        Self::WinRt(e)
    }
}

impl From<std::io::Error> for TranscodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for TranscodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WinRt(e) => e.message().fmt(f),
            Self::Io(e) => e.fmt(f),
            Self::NonUtf8Path => "A non-ut8 path was used by this library.".fmt(f),
            Self::PrepareTranscodeFailure(TranscodeFailureReason::None) => {
                "No transcode prepare failure.".fmt(f)
            }
            Self::PrepareTranscodeFailure(TranscodeFailureReason::Unknown) => {
                "Unknown transcode prepare failure.".fmt(f)
            }
            Self::PrepareTranscodeFailure(TranscodeFailureReason::InvalidProfile) => {
                "Transcode prepare failure, invalid profile".fmt(f)
            }
            Self::PrepareTranscodeFailure(TranscodeFailureReason::CodecNotFound) => {
                "Transcode prepare failure, unknown codec".fmt(f)
            }
            Self::PrepareTranscodeFailure(_) => "Transcode prepare failure".fmt(f),
        }
    }
}

impl std::error::Error for TranscodeError {}
