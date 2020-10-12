pub use crate::bindings::windows::media::transcoding::TranscodeFailureReason;

/// Overall Result Type
pub type TranscodeResult<T> = Result<T, TranscodeError>;

/// Overall Error Type
#[derive(Debug)]
pub enum TranscodeError {
    /// A WinRT error
    WinRt(winrt::Error),

    /// An I/O Error
    Io(std::io::Error),

    /// A path was used with this library that wasn't utf8.
    /// Note: Because of limitations in [`winrt::HString`], we cannot use aritrary byte strings
    NonUtf8Path,

    /// Failed to prepare a transcode
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
