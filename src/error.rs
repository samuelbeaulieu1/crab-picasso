use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to download image from url, {reason}"))]
    FailedToDownloadImage { reason: String },
    #[snafu(display("Filetype ({filetype}) is unsupported"))]
    UnsupportedFileType { filetype: String },
    #[snafu(display("Image is too big ({size} bytes)"))]
    ImageTooBig { size: u64 },
}