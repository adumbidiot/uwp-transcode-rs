winrt::build!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::media::transcoding::*
);

fn main() {
    build();
}
