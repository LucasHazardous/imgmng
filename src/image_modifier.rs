use super::Modifications;
use std::{error::Error, path::Path};

extern crate image;
use image::GenericImageView;

pub fn modify_image(
    filename: &str,
    modifications: &Modifications,
    target_path: &str,
) -> Result<(), Box<dyn Error>> {
    let img = image::open(Path::new(&format!("{}{}", target_path, filename)))?;

    let mut img = img.resize(
        (img.dimensions().0 as f32 * modifications.resize) as u32,
        (img.dimensions().1 as f32 * modifications.resize) as u32,
        image::imageops::Lanczos3,
    );

    if modifications.invert {
        img.invert();
    }

    img.save_with_format(
        Path::new(&format!("{}modified/{}", target_path, filename)),
        image::ImageFormat::Jpeg,
    )?;

    Ok(())
}
