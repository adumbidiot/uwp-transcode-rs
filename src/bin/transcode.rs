use uwp_transcode::{
    File as UwpFile,
    MediaProfile as UwpMediaProfile,
    Transcoder as UwpTranscoder,
    VideoQuality as UwpVideoQuality,
};

#[derive(argh::FromArgs)]
#[argh(description = "A Windows program to transcode media")]
struct Args {
    #[argh(positional)]
    source: String,

    #[argh(positional)]
    dest: String,
}

fn main() {
    let args = argh::from_env();
    futures::executor::block_on(async_main(args));
}

async fn async_main(args: Args) {
    let transcoder = match UwpTranscoder::new() {
        Ok(transcoder) => transcoder,
        Err(e) => {
            eprintln!("Failed to create transcoder: {}", e);
            return;
        }
    };

    println!("Transcoding '{}' to '{}'...", args.source, args.dest);

    let input_file = match UwpFile::open(&args.source).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open input file: {}", e);
            return;
        }
    };

    let output_file_options = Default::default();
    let output_file = match UwpFile::create(&args.dest, output_file_options).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open input file: {}", e);
            return;
        }
    };

    let target_profile = match UwpMediaProfile::create_hevc(UwpVideoQuality::HD1080p) {
        Ok(profile) => profile,
        Err(e) => {
            eprintln!("Failed to create encoding profile: {}", e);
            return;
        }
    };

    let prepared_transcode = match transcoder
        .prepare_transcode(&input_file, &output_file, &target_profile)
        .await
    {
        Ok(prepared) => prepared,
        Err(e) => {
            eprintln!("Failed to prepare transcode: {}", e);
            return;
        }
    };

    if let Err(e) = prepared_transcode
        .transcode_with_progress(|progress| {
            println!("{}%", progress);
        })
        .await
    {
        println!("Failed to start transcode: {}", e);
        return;
    };
    
    println!("Complete!");
}
