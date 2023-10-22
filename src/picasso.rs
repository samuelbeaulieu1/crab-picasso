use std::{
    fs::File, 
    io::{copy, Cursor, Read, BufReader}
};

use dotenv::dotenv;
use image::{GenericImageView, RgbImage};
use openai_api_rust::{
    OpenAI,
    Auth,
};
use reqwest;
use uuid::Uuid;

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
    fn check_file_type(&self, buf: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
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

    fn save_as_rgba(&self, image_buffer: &[u8]) -> Result<File, Box<dyn std::error::Error>> {
        let img = image::io::Reader::new(Cursor::new(&image_buffer)).with_guessed_format()?.decode()?;

        let filename = Uuid::new_v4();
        let path = format!("/tmp/{filename}.png");
        img.into_rgba8().save(&path)?;

        Ok(File::open(path)?)
    }

    pub fn download_image(&self, url: &str) -> Result<File, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(url)?;

        if resp.status() != reqwest::StatusCode::OK {
            return Err(Box::new(error::Error::FailedToDownloadImage { reason: resp.text()? }));
        }

        let resp_bytes = resp.bytes()?;
        self.check_file_type(&resp_bytes.to_vec())?;

        Ok(self.save_as_rgba(&resp_bytes)?)
    }

    pub fn load_image(&self, path: &str) -> Result<File, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut buf = Vec::new();
        let mut buf_reader = BufReader::new(&file);
        buf_reader.read_to_end(&mut buf)?;

        self.check_file_type(&buf)?;

        Ok(self.save_as_rgba(&buf)?)
    }
}