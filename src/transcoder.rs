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

#[derive(Debug, Clone)]
pub struct MediaProfile {
    profile: MediaEncodingProfile,
}

impl MediaProfile {
    pub fn create_hevc(video_quality: VideoQuality) -> TranscodeResult<Self> {
        let profile = MediaEncodingProfile::create_hevc(video_quality.into())?;

        Ok(MediaProfile { profile })
    }

    pub fn create_mp3(audio_quality: AudioQuality) -> TranscodeResult<Self> {
        let profile = MediaEncodingProfile::create_mp3(audio_quality.into())?;

        Ok(MediaProfile { profile })
    }

    pub fn as_media_encoding_profile(&self) -> &MediaEncodingProfile {
        &self.profile
    }
}

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
    Uhd2160p,
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

pub struct PreparedTranscode {
    operation: PrepareTranscodeResult,
}

impl PreparedTranscode {
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

    pub async fn transcode(&self) -> TranscodeResult<()> {
        self.operation.transcode_async()?.await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Transcoder {
    transcoder: MediaTranscoder,
}

impl Transcoder {
    pub fn new() -> TranscodeResult<Self> {
        let transcoder = MediaTranscoder::new()?;

        Ok(Transcoder { transcoder })
    }

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

    pub fn as_media_transcoder(&self) -> &MediaTranscoder {
        &self.transcoder
    }
}

impl Transcoder {
    pub fn setup_transcode() {}
}
