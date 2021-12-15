use anyhow::Context;
use anyhow::Result as AnyResult;
use image::io::Reader as ImageReader;
use image::GenericImageView;

// https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
// https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

mod conversions;
use conversions::{image_to_tensor, tensor_to_image};

mod mirnet_model;
use mirnet_model::MirnetModel;

fn main() -> AnyResult<()> {
    let model = MirnetModel::new("model")?;

    // Create input variables for our addition
    let img_reader = ImageReader::open("79.png").context("Failed to read image")?;
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
