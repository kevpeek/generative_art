mod masks;

extern crate image;

use crate::masks::grayscale;

fn main() -> anyhow::Result<()> {
    let image = image::open("input.png")?;
    println!("loaded image");

    let mask = masks::grayscale();
    let new_image = mask(image.into_rgba8());

    println!("done processing image");
    new_image.save("output.png")?;
    Ok(())
}

