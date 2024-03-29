use std::fs::File;
use std::io::{Write, Error};

use crate::vec3::Color;

#[derive(Debug)]
struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

pub fn format_color(color: &Color, samples: u32) -> String {
    let converted = generate_color(color, samples);
    format!("{} {} {}\n", converted.r, converted.g, converted.b)
}

pub fn write_color(file: &mut File, color: &Color, samples: u32) -> Result<(), Error> {
    let converted = generate_color(color, samples);
    write!(file, "{} {} {}", converted.r, converted.g, converted.b)?;
    Ok(())
}

fn generate_color(color: &Color, samples: u32) -> RGB {
    let scale = 1. / samples as f32;
    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();

    RGB {
        r: ((256. * r.clamp(0., 0.999)).floor()) as u32,
        g: ((256. * g.clamp(0., 0.999)).floor()) as u32,
        b: ((256. * b.clamp(0., 0.999)).floor()) as u32,
    }
}

