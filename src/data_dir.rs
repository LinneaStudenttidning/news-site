use std::{env, fs};

use crate::{defaults::DATA_DIR, error::Error};
use dotenvy::dotenv;

pub fn get_about_us() -> String {
    dotenv().ok();
    let data_dir = env::var("DATA_DIR").unwrap_or(DATA_DIR.into());
    let about_us_md_path = format!("{data_dir}/about_us.md");
    fs::read_to_string(about_us_md_path)
        .unwrap_or("".to_string())
        .clone()
        .to_string()
}

pub fn edit_about_us(new_about_us: String) -> Result<(), Error> {
    let data_dir = env::var("DATA_DIR").unwrap_or(DATA_DIR.into());
    let about_us_md_path = format!("{data_dir}/about_us.md");

    fs::write(about_us_md_path, new_about_us).map_err(Error::from)
}
