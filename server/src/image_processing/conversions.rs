use std::num::TryFromIntError;

use image::GenericImageView;
use image::RgbImage;
use image::Rgba;
use tensorflow::Tensor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("invalid dimensions, expected [1, w, h, 3]")]
    InvalidDimensions,

    #[error("invalid dimension {dimension} ({value}) : {source}")]
    InvalidDimension {
        dimension: String,
        value: u64,
        source: TryFromIntError,
    },
}

/// Convert an RGB image (Rgba is taken as input but the alpha layer is ignored) to a tensor of dimension
/// `[1, height, width, 3]` using color values between 0 and 1.
pub fn image_to_tensor<InnerImageView>(
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
pub fn tensor_to_image(tensor: &Tensor<f32>) -> Result<RgbImage, ConversionError> {
    let dims = tensor.dims();
    if dims.len() != 4 || dims[0] != 1 || dims[3] != 3 {
        return Err(ConversionError::InvalidDimensions);
    }

    let output_height: u32 = dims[1]
        .try_into()
        .map_err(|e| ConversionError::InvalidDimension {
            dimension: "Height".into(),
            value: dims[1],
            source: e,
        })?;
    let output_width: u32 = dims[2]
        .try_into()
        .map_err(|e| ConversionError::InvalidDimension {
            dimension: "Width".into(),
            value: dims[1],
            source: e,
        })?;

    let mut image = RgbImage::new(output_width, output_height);

    for (i, pixel) in image.pixels_mut().enumerate() {
        pixel.0[0] = (tensor[3 * i] * 255.0) as u8;
        pixel.0[1] = (tensor[3 * i + 1] * 255.0) as u8;
        pixel.0[2] = (tensor[3 * i + 2] * 255.0) as u8;
    }

    Ok(image)
}
