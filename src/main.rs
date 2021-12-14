use anyhow::Context;
use anyhow::Result as AnyResult;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use image::RgbImage;
use image::Rgba;
use std::path::Path;
use tensorflow::Graph;
use tensorflow::SavedModelBundle;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Tensor;
use tensorflow::DEFAULT_SERVING_SIGNATURE_DEF_KEY;

// https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
// https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

/// Convert an RGB image (Rgba is taken as input but the alpha layer is ignored) to a tensor of dimension
/// `[1, height, width, 3]` using color values between 0 and 1.
fn image_to_tensor<InnerImageView>(
    img: &impl GenericImageView<Pixel = Rgba<u8>, InnerImageView = InnerImageView>,
) -> Tensor<f32> {
    let mut tensor = Tensor::new(&[1, img.height().into(), img.width().into(), 3]);

    for (i, (_, _, pixel)) in img.pixels().enumerate() {
        tensor[3 * i] = pixel.0[0] as f32 / 255.0;
        tensor[3 * i + 1] = pixel.0[1] as f32 / 255.0;
        tensor[3 * i + 2] = pixel.0[2] as f32 / 255.0;
    }

    tensor
}

/// Convert an RGB image (Rgba is taken as input but the alpha layer is ignored) to a tensor of dimension
/// `[1, height, width, 3]` using color values between 0 and 1.
fn tensor_to_image(tensor: &Tensor<f32>) -> AnyResult<RgbImage> {
    let dims = tensor.dims();
    let output_height: u32 = dims[1].try_into()?;
    let output_width: u32 = dims[2].try_into()?;

    let mut image = RgbImage::new(output_width, output_height);

    for (i, pixel) in image.pixels_mut().enumerate() {
        pixel.0[0] = (tensor[3 * i] * 255.0) as u8;
        pixel.0[1] = (tensor[3 * i + 1] * 255.0) as u8;
        pixel.0[2] = (tensor[3 * i + 2] * 255.0) as u8;
    }

    Ok(image)
}

struct MirnetModel {
    graph: Graph,
    bundle: SavedModelBundle,
}

impl MirnetModel {
    pub fn new(model_dir: impl AsRef<Path>) -> AnyResult<MirnetModel> {
        let mut graph = Graph::new();
        let bundle =
            SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_dir)?;

        Ok(MirnetModel { graph, bundle })
    }

    pub fn run(&self, input: &Tensor<f32>) -> AnyResult<Tensor<f32>> {
        let signature = self
            .bundle
            .meta_graph_def()
            .get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)?;

        let (_, input_info) = signature
            .inputs()
            .into_iter()
            .next()
            .context("No input found")?;
        let op_input = &self
            .graph
            .operation_by_name_required(&input_info.name().name)?;

        let (_, output_info) = signature
            .outputs()
            .into_iter()
            .next()
            .context("No output found")?;
        let op_output = &self
            .graph
            .operation_by_name_required(&output_info.name().name)?;

        let mut args = SessionRunArgs::new();
        args.add_feed(op_input, 0, &input);
        let token_output = args.request_fetch(op_output, 0);

        self.bundle.session.run(&mut args)?;

        let output = args.fetch(token_output)?;
        Ok(output)
    }
}

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
