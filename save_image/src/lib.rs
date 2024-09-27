extern crate image;

use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub fn pgm_p5(width: u32, height: u32, data: &[u8], filename: &str) -> std::io::Result<()> {
    let mut file: File = File::create(format!("{}.pgm", filename))?;
    // PGM headers
    writeln!(file, "P5")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;
    // write image data
    file.write_all(data)?;
    Ok(())
}

#[allow(dead_code)]
pub fn pgm_p6(width: u32, height: u32, data: &[u8], filename: &str) -> std::io::Result<()> {
    let mut file: File = File::create(format!("{}.pgm", filename))?;
    let buf = map_to_palette(data, palette);

    // PGM headers
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    // write image data
    file.write_all(&buf)?;
    Ok(())
}

#[allow(dead_code)]
pub fn png(width: u32, height: u32, data: &[u8], filename: &str) -> std::io::Result<()> {
    let buf = map_to_palette(data, palette);
    image::save_buffer_with_format(
        format!("{}.png", filename),
        &buf,
        width,
        height,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .unwrap();
    Ok(())
}

fn map_to_palette<T: Copy, F: Fn(T) -> Vec<T>>(data: &[T], palette: F) -> Vec<T> {
    data.into_iter()
        .map(|&i| palette(i))
        .fold(vec![], |mut acc, cur| {
            acc.extend(cur);
            acc
        })
}

fn palette(i: u8) -> Vec<u8> {
    match i {
        0 => vec![255, 255, 255],
        _ => vec![(3 * i) % 255, (5 * i) % 255, (7 * i) % 255],
    }
}
