use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use anyhow::Context;
use anyhow::Result as AnyResult;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::conversions::{image_to_tensor, tensor_to_image};
use crate::mirnet_model::MirnetModel;

pub fn run(path: PathBuf) -> AnyResult<()> {
    let model = MirnetModel::new("model")?;

    // Create input variables for our addition
    let img_reader = ImageReader::open(path).context("Failed to read image")?;
    let img = img_reader.decode().context("Failed to decode image")?;

    println!("Opened image {}x{}", img.height(), img.width());

    println!("Reading pixels to tensor...");
    let input = image_to_tensor(&img);

    println!("Running...");
    let output = model.run(&input)?;
    let output_image = tensor_to_image(&output)?;

    output_image.save_with_format("out.png", image::ImageFormat::Png)?;

    println!("out: {:?}", &output);

    Ok(())
}
