use std::{
    fs::File, 
    io::{copy, Cursor}
};

use dotenv::dotenv;
use openai_api_rust::{
    OpenAI,
    Auth,
};
use reqwest;

use crate::error;


pub struct Picasso {
    pub openai: OpenAI,
}

pub fn new() -> Picasso {
    dotenv().expect("Failed to load .env");
    env_logger::init();

    let auth = Auth::from_env().unwrap();
    Picasso {
        openai: OpenAI::new(auth, "https://api.openai.com/v1/")
    }
}

impl Picasso {
    fn check_file_type(&self, buf: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        match infer::get(&buf) {
            Some(filetype) => {
                match filetype.mime_type() {
                    "image/png" => Ok(()),
                    _ => Err(Box::new(error::Error::UnsupportedFileType { filetype: filetype.mime_type().to_string() }))
                }
            },
            None => Err(Box::new(error::Error::UnsupportedFileType { filetype: String::from("") }))
        }
    }

    pub fn download_image(&self, url: &str) -> Result<File, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(url)?;

        if resp.status() != reqwest::StatusCode::OK {
            return Err(Box::new(error::Error::FailedToDownloadImage { reason: resp.text()? }));
        }

        let resp_bytes = resp.bytes()?;
        self.check_file_type(resp_bytes.to_vec())?;

        let mut file = tempfile::tempfile()?;
        let mut content = Cursor::new(resp_bytes);
        copy(&mut content, &mut file)?;

        Ok(file)
    }

    pub fn load_image(&self, path: &str) -> Result<File, Box<dyn std::error::Error>> {
        todo!()
    }
}