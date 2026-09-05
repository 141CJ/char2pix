use std::str::FromStr;

use anyhow::Context;
use clap::{Parser, ValueEnum};
use opencv::core::{self, MatExprTraitConst, MatTrait, MatTraitConst};
use opencv::imgcodecs;

#[derive(PartialEq, Copy, Clone, Debug, ValueEnum)]
enum Mode {
    Encode,
    Decode,
}

impl FromStr for Mode {
    type Err = String;
    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "encode" => Ok(Mode::Encode),
            "decode" => Ok(Mode::Decode),
            _ => Err(format!("'{}' is invalid", input)),
        }
    }
}

#[derive(Parser)]
struct Cli {
    mode: Mode,
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let path = std::path::Path::new(&args.input);
    match args.mode {
        Mode::Encode => {
            let data = if path.exists() {
                std::fs::read_to_string(&args.input)
                    .context(format!("Failed to read file '{}'", args.input))?
            } else {
                args.input
            };
            encode(data)?
        }
        Mode::Decode => decode(args.input)?,
    };
    Ok(())
}

fn decode(img_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let img = imgcodecs::imread(img_name, imgcodecs::IMREAD_COLOR)?;
    let size = img.rows() * img.cols();
    for i in 0..size {
        let color = img.at::<core::Vec3b>(i).unwrap();
        print!(
            "{}{}{}",
            color[2] as char, color[1] as char, color[0] as char
        );
    }
    Ok(())
}

fn encode(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let text_array = text.chars();

    let pixels_needed = (text.len() + 2) / 3;
    let width = (pixels_needed as f64).sqrt().ceil() as i32;
    let height = ((pixels_needed as f64) / (width as f64)).ceil() as i32;
    let mut img = core::Mat::zeros(height, width, core::CV_8UC3)?.to_mat()?;

    let mut colors: Vec<u8> = Vec::with_capacity(3);

    let mut index = 0;
    for i in text_array.into_iter() {
        colors.push(i as u8);
        if colors.len() == 3 {
            let pixel = img.at_mut::<core::Vec3b>(index)?;
            pixel[2] = colors[0];
            pixel[1] = colors[1];
            pixel[0] = colors[2];
            colors.clear();
            index += 1;
        }
    }
    if !colors.is_empty() {
        let pixel = img.at_mut::<core::Vec3b>(index)?;
        pixel[2] = colors.get(0).copied().unwrap_or(0);
        pixel[1] = colors.get(1).copied().unwrap_or(0);
        pixel[0] = colors.get(2).copied().unwrap_or(0);
    }

    imgcodecs::imwrite("pixels.png", &img, &core::Vector::new())?;
    Ok(())
}
