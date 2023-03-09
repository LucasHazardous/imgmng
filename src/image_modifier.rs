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

    let img_x = img.dimensions().0 as f32;
    let img_y = img.dimensions().1 as f32;

    if modifications.special {
        let mut imgbuf = image::ImageBuffer::from(img.to_rgb8());
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let pixel_rgb = pixel.0;
            let bonus = ((x as f32 / img_x + y as f32 / img_y) * 255.0 * 0.5) as u32;

            *pixel = image::Rgb([
                ((pixel_rgb[0] as u32 + bonus) % 255) as u8,
                ((pixel_rgb[1] as u32 + bonus) % 255) as u8,
                ((pixel_rgb[2] as u32 + bonus) % 255) as u8,
            ]);
        }

        imgbuf.save_with_format(
            Path::new(&format!("{}modified/{}", target_path, filename)),
            image::ImageFormat::Jpeg,
        )?;
    } else {
        img.save_with_format(
            Path::new(&format!("{}modified/{}", target_path, filename)),
            image::ImageFormat::Jpeg,
        )?;
    }

    Ok(())
}
