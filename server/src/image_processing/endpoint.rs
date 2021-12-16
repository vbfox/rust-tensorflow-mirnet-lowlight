use crate::authenticate;
use crate::users::UserDb;

use super::{image_to_tensor, tensor_to_image, MirnetModel};
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use futures::{StreamExt, TryStreamExt};
use image::io::Reader as ImageReader;
use image::png::PngEncoder;
use image::{EncodableLayout, ImageEncoder};
use std::io::{Cursor, Write};
use thiserror::Error;
use tracing::info_span;
use tracing::instrument;
use tracing::trace;

async fn get_input_bytes(mut payload: Multipart) -> Result<Vec<u8>, actix_web::Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        if field.name() == "input" {
            let mut bytes: Vec<u8> = Vec::new();

            while let Some(chunk) = field.next().await {
                let data = chunk?;
                trace!("Writing {} bytes", data.len());
                bytes.write_all(&data)?;
            }

            trace!("Found input with {} bytes", bytes.len());

            return Ok(bytes);
        }
    }

    Err(actix_web::error::ErrorBadRequest("Field not found"))
}

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("bad request: {0}")]
    ErrorBadRequest(String),

    #[error("internal server error: {0}")]
    ErrorInternalServerError(String),
}

impl From<ProcessError> for actix_web::Error {
    fn from(process_error: ProcessError) -> Self {
        match process_error {
            ProcessError::ErrorBadRequest(m) => actix_web::error::ErrorBadRequest(m),
            ProcessError::ErrorInternalServerError(m) => {
                actix_web::error::ErrorInternalServerError(m)
            }
        }
    }
}

fn process_image_blocking(input_bytes: Vec<u8>) -> Result<Vec<u8>, ProcessError> {
    let span = info_span!("Processing image request");
    let _ = span.enter();

    let input_image = ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()
        .map_err(|e| ProcessError::ErrorBadRequest(format!("Unable to guess format: {:?}", e)))?
        .decode()
        .map_err(|e| ProcessError::ErrorBadRequest(format!("Invalid image: {:?}", e)))?;

    let input_tensor = image_to_tensor(&input_image);
    drop(input_image);

    let model = MirnetModel::new("model").map_err(|e| {
        ProcessError::ErrorInternalServerError(format!("Can't initialize model: {:?}", e))
    })?;

    let output_tensor = model.run(&input_tensor).map_err(|e| {
        ProcessError::ErrorInternalServerError(format!("Error running model: {:?}", e))
    })?;
    let output_image = tensor_to_image(&output_tensor).map_err(|e| {
        ProcessError::ErrorInternalServerError(format!("Can't convert output: {:?}", e))
    })?;

    let mut output_bytes = Vec::<u8>::new();
    PngEncoder::new(&mut output_bytes)
        .write_image(
            output_image.as_bytes(),
            output_image.width(),
            output_image.height(),
            image::ColorType::Rgb8,
        )
        .map_err(|e| {
            ProcessError::ErrorInternalServerError(format!("Can't encode output: {:?}", e))
        })?;

    Ok(output_bytes)
}

#[instrument(skip(payload, id, user_db))]
pub async fn process_image(
    payload: Multipart,
    id: Identity,
    user_db: web::Data<UserDb>,
) -> Result<HttpResponse, actix_web::Error> {
    authenticate!(&id, &user_db);
    let input_bytes = get_input_bytes(payload).await?;
    let output_bytes = web::block(|| process_image_blocking(input_bytes)).await??;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("image/png")
        .body(output_bytes))
}
