use std::fs::File;
use std::io::{Write, Error};

use crate::vec3::Color;

pub fn print_color(color: &Color) {
    println!("{} {} {}", (color.x() * 255.999).floor() as u32,
                         (color.y() * 255.999).floor() as u32,
                         (color.z() * 255.999).floor() as u32);
}

pub fn write_color(file: &mut File, color: &Color) -> Result<(), Error> {
    write!(file, "{} {} {}", (color.x() * 255.999).floor() as u32,
                             (color.y() * 255.999).floor() as u32,
                             (color.z() * 255.999).floor() as u32)?;
    Ok(())
}
