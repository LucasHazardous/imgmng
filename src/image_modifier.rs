use std::{path::Path, error::Error};
use super::Modifications;

extern crate image;
use image::GenericImageView;

pub fn modify_image(filename: &str, modifications: &Modifications) -> Result<(), Box<dyn Error>> {
    let img = image::open(Path::new(filename))?;
    
    let img = img.resize((img.dimensions().0 as f32 * modifications.resize) as u32, (img.dimensions().1 as f32 * modifications.resize) as u32, image::imageops::Lanczos3);

    img.save_with_format(Path::new(&format!("./modified/{}", filename)), image::ImageFormat::Jpeg)?;

    Ok(())
}
