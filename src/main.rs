use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use anyhow::Context;
use image::DynamicImage;
use image::GenericImage;
use image::Rgb;
use image::RgbImage;
use image::Rgba;
use tensorflow::Code;
use tensorflow::Graph;
use tensorflow::SavedModelBundle;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Status;
use tensorflow::Tensor;
use tensorflow::DEFAULT_SERVING_SIGNATURE_DEF_KEY;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use anyhow::Result as AnyResult;

// https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
// https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

fn main() -> AnyResult<()> {
    let model_dir = "model";

    // Load the saved model exported by zenn_savedmodel.py.
    let mut graph = Graph::new();
    let bundle =
        SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_dir)?;
    let session = &bundle.session;

    // get in/out operations
    let signature = bundle
        .meta_graph_def()
        .get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)?;


    let (input_name, input_info) = signature.inputs().into_iter().next().context("No input found")?;
    println!("Using input: {}", input_name);
    let op_input = &graph.operation_by_name_required(&input_info.name().name)?;

    let (output_name, output_info) = signature.outputs().into_iter().next().context("No output found")?;
    println!("Using output: {}", output_name);
    let op_output = &graph.operation_by_name_required(&output_info.name().name)?;

    // Create input variables for our addition
    let img_reader = ImageReader::open("79.png").context("Failed to read image")?;
    let img = img_reader.decode().context("Failed to decode image")?;
    println!("Opened image {}x{}", img.height(), img.width());

    println!("Reading pixels to tensor...");
    let mut input = Tensor::new(&[1, img.height().into(), img.width().into(), 3]);
    for (i, (_, _, pixel)) in img.pixels().enumerate() {
        input[3 * i] = pixel.0[0] as f32 / 255.0;
        input[3 * i + 1] = pixel.0[1] as f32 / 255.0;
        input[3 * i + 2] = pixel.0[2] as f32 / 255.0;
    }

    println!("Configuring the graph...");
    // Run the graph.
    let mut args = SessionRunArgs::new();
    args.add_feed(op_input, 0, &input);
    let token_output = args.request_fetch(op_output, 0);

    println!("Running...");
    session.run(&mut args)?;

    // Check the output.
    println!("Done !");
    let output: Tensor<f32> = args.fetch(token_output)?;
    let dims = output.dims();
    let output_height: u32 = dims[1].try_into()?;
    let output_width: u32 = dims[2].try_into()?;
    let mut output_image = RgbImage::new(output_width, output_height);

    for (i, pixel) in output_image.pixels_mut().enumerate() {
        pixel.0[0] = (output[3*i] *255.0) as u8;
        pixel.0[1] = (output[3*i+1]*255.0) as u8;
        pixel.0[2] = (output[3*i+2]*255.0) as u8;
    }
/*
    for x in 0..output_width {
        for y in 0..output_height {

            let r = output.get(&[0, y.into(), x.into(), 0]);
            let g = output.get(&[0, y.into(), x.into(), 1]);
            let b = output.get(&[0, y.into(), x.into(), 2]);
            println!("R {}, G {}, B {}", r, g, b);
            let r8 = r as u8;
            let g8 = g as u8;
            let b8 = b as u8;
            println!("R {}, G {}, B {}", r8, g8, b8);

            output_image.put_pixel(x, y, Rgb([r8, g8, b8]));
        }
    }
*/
    output_image.save_with_format("out.png", image::ImageFormat::Png)?;



    println!("out: {:?}", &output);

    Ok(())
}
