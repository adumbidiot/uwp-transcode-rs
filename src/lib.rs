pub mod error;
pub mod file;
pub mod transcoder;

pub use self::{
    error::{
        TranscodeError,
        TranscodeResult,
    },
    file::File,
    transcoder::{
        AudioQuality,
        MediaProfile,
        Transcoder,
        VideoQuality,
    },
};
pub use windows_media_transcoding_bindings as bindings;

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        futures::executor::block_on(it_works_async()).unwrap();

        let output_path = Path::new("./test_output/oof1.mp4");
        assert!(output_path.exists());
        assert!(output_path.is_file());

        let data = std::fs::read(output_path).unwrap();
        assert!(!data.is_empty());
    }

    async fn it_works_async() -> Result<(), TranscodeError> {
        let transcoder = Transcoder::new()?;

        let input_file = File::open("./test_data/oof.mp4").await?;
        let output_file = File::create("./test_output/oof1.mp4", Default::default()).await?;
        let target_profile = MediaProfile::create_hevc(VideoQuality::HD720p)?;

        let prepared_transcode = transcoder
            .prepare_transcode(&input_file, &output_file, &target_profile)
            .await?;

        prepared_transcode
            .transcode_with_progress(|progress| {
                dbg!(progress);
            })
            .await?;

        Ok(())
    }
}
