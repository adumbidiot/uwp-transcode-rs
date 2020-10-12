pub use crate::bindings::windows::{
    foundation::AsyncActionProgressHandler,
    media::transcoding::MediaTranscoder,
};
use crate::{
    File,
    TranscodeError,
    TranscodeResult,
};
pub use windows_media_transcoding_bindings::windows::media::{
    media_properties::{
        AudioEncodingQuality,
        MediaEncodingProfile,
        VideoEncodingQuality,
    },
    transcoding::PrepareTranscodeResult,
};

/// A media profile used for transcoding. Cloning this produces another handle to it.
#[derive(Debug, Clone)]
pub struct MediaProfile {
    profile: MediaEncodingProfile,
}

impl MediaProfile {
    /// Create a HEVC profile with the given video quality.
    pub fn create_hevc(video_quality: VideoQuality) -> TranscodeResult<Self> {
        let profile = MediaEncodingProfile::create_hevc(video_quality.into())?;

        Ok(MediaProfile { profile })
    }

    /// Create an Mp3 profile with the given audio quality.
    pub fn create_mp3(audio_quality: AudioQuality) -> TranscodeResult<Self> {
        let profile = MediaEncodingProfile::create_mp3(audio_quality.into())?;

        Ok(MediaProfile { profile })
    }

    /// Get the inner [`MediaEncodingProfile`] for this object
    pub fn as_media_encoding_profile(&self) -> &MediaEncodingProfile {
        &self.profile
    }
}

/// A representation of video quality
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum VideoQuality {
    Auto,
    HD1080p,
    HD720p,
    Wvga,
    Ntsc,
    Pal,
    Vga,
    Qvga,

    // NOTE: May not be present on all platforms
    Uhd2160p,

    // Note: May not be present on all platforms
    Uhd4320p,
}

impl Into<VideoEncodingQuality> for VideoQuality {
    fn into(self) -> VideoEncodingQuality {
        match self {
            Self::Auto => VideoEncodingQuality::Auto,
            Self::HD1080p => VideoEncodingQuality::HD1080p,
            Self::HD720p => VideoEncodingQuality::HD720p,
            Self::Wvga => VideoEncodingQuality::Wvga,
            Self::Ntsc => VideoEncodingQuality::Ntsc,
            Self::Pal => VideoEncodingQuality::Pal,
            Self::Vga => VideoEncodingQuality::Vga,
            Self::Qvga => VideoEncodingQuality::Qvga,
            Self::Uhd2160p => VideoEncodingQuality::Uhd2160p,
            Self::Uhd4320p => VideoEncodingQuality::Uhd4320p,
        }
    }
}

/// A representation of audio quality
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum AudioQuality {
    Auto,
    High,
    Medium,
    Low,
}

impl Into<AudioEncodingQuality> for AudioQuality {
    fn into(self) -> AudioEncodingQuality {
        match self {
            Self::Auto => AudioEncodingQuality::Auto,
            Self::High => AudioEncodingQuality::High,
            Self::Medium => AudioEncodingQuality::Medium,
            Self::Low => AudioEncodingQuality::Low,
        }
    }
}

/// A Transcoding Operation thats ready to be started
pub struct PreparedTranscode {
    operation: PrepareTranscodeResult,
}

impl PreparedTranscode {
    /// Begin transcoding with a progress callback
    pub async fn transcode_with_progress<F: FnMut(f64) + 'static>(
        &self,
        mut f: F,
    ) -> TranscodeResult<()> {
        let operation = self.operation.transcode_async()?;
        let handler = AsyncActionProgressHandler::new(move |_handler, progress| {
            f(*progress);
            Ok(())
        });
        operation.set_progress(handler)?;

        operation.await?;

        Ok(())
    }

    /// Begin transcoding
    pub async fn transcode(&self) -> TranscodeResult<()> {
        self.operation.transcode_async()?.await?;

        Ok(())
    }
}

/// A Transcoder
#[derive(Debug, Clone)]
pub struct Transcoder {
    transcoder: MediaTranscoder,
}

impl Transcoder {
    /// Create a new transcoder
    pub fn new() -> TranscodeResult<Self> {
        let transcoder = MediaTranscoder::new()?;

        Ok(Transcoder { transcoder })
    }

    /// Prepare a transcode operation and verify it
    pub async fn prepare_transcode(
        &self,
        input_file: &File,
        output_file: &File,
        target_profile: &MediaProfile,
    ) -> TranscodeResult<PreparedTranscode> {
        let operation = self
            .transcoder
            .prepare_file_transcode_async(
                &input_file.file,
                &output_file.file,
                &target_profile.profile,
            )?
            .await?;

        if !operation.can_transcode()? {
            return Err(TranscodeError::PrepareTranscodeFailure(
                operation.failure_reason()?,
            ));
        }

        Ok(PreparedTranscode { operation })
    }

    /// Get the inner [`MediaTranscoder`] from this object.
    pub fn as_media_transcoder(&self) -> &MediaTranscoder {
        &self.transcoder
    }
}
