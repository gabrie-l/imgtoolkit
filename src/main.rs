extern crate rayon;
use rayon::prelude::*;
use std::{
    ffi::OsString,
    fs::{self, ReadDir},
    path::PathBuf,
};
mod resize;
use clap::{arg, command, value_parser};
use image::{self, GenericImageView};
use std::u32;

fn filenames(path: &String) -> ReadDir {
    fs::read_dir(path).unwrap()
}

fn resize_img(
    height: Option<&u32>,
    width: Option<&u32>,
    percent: Option<&u32>,
    infile: OsString,
    outfile: Option<&String>,
) {
    let img = image::open(&infile).expect("Failed to open image");
    let (h, w) = img.dimensions();
    let mut dims: Vec<u32> = vec![h, w];
    if let Some(percent) = percent {
        let new_h = (h as f32) * (*percent as f32) / 100.0;
        let new_w = (w as f32) * (*percent as f32) / 100.0;
        dims[0] = new_h as u32;
        dims[1] = new_w as u32;
    } else if let Some(height) = height {
        dims[0] = *height;
        if let Some(width) = width {
            dims[1] = *width;
        }
    }
    // if let
    println!("Final Dimensions: {} X {}", dims[0], dims[1]);

    // Resize the image
    let resized_img = resize::resize_image(&img, dims[0], dims[1]);

    // Save the resized image
    if let Some(outfile) = outfile {
        resized_img.save(outfile).expect("Failed to save image");
        println!("Image resized and saved to {}", outfile);
    } else {
        resized_img.save(infile).expect("Failed to save image");
        println!("Image resized with same name");
    }
}

fn main() {
    // Open an image file

    let matches = command!()
        .arg(arg!(-i --input <FILE>).help("Input file/directory"))
        .arg(
            arg!(-o --output <FILE>)
                .required(false)
                .help("Output file/directory"),
        )
        .arg(
            arg!(-H --height <VALUE>)
                .value_parser(value_parser!(u32))
                .required(false)
                .help("Height"),
        )
        .arg(
            arg!(-W --width <VALUE>)
                .value_parser(value_parser!(u32))
                .required(false)
                .help("Width"),
        )
        .arg(
            arg!(-p --percentage <VALUE>)
                .value_parser(value_parser!(u32))
                .required(false)
                .help("Percentage"),
        )
        .get_matches();

    let infile = matches.get_one::<String>("input").expect("required");
    let outfile = matches.get_one::<String>("output");
    let height = matches.get_one::<u32>("height");
    let width = matches.get_one::<u32>("width");
    let percent = matches.get_one::<u32>("percentage");
    match fs::metadata(infile) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let files: Vec<_> = filenames(&infile).collect();
                files.par_iter().for_each(|f| {
                    let filename = f.as_ref().expect("Not a valid file").file_name();
                    let mut input = PathBuf::new();
                    input.push(infile);
                    input.push(filename);
                    println!("{:?} this is the input file", input);
                    resize_img(
                        height,
                        width,
                        percent,
                        input.as_path().as_os_str().into(),
                        outfile,
                    )
                })
            } else if metadata.is_file() {
                resize_img(height, width, percent, infile.into(), outfile);
            }
        }
        Err(_) => println!("Not a valid file/directory"),
    }
}
