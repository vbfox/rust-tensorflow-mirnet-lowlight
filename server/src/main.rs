use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use anyhow::Context;
use anyhow::Result as AnyResult;
use futures::{StreamExt, TryStreamExt};
use image::io::Reader as ImageReader;
use image::{GenericImageView, EncodableLayout, ImageEncoder};
use image::png::PngEncoder;
use std::fs::OpenOptions;
use std::io::{self, Cursor, Write, BufWriter};
use std::path::PathBuf;
use structopt::StructOpt;

// https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
// https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

pub mod conversions;
use conversions::{image_to_tensor, tensor_to_image};

pub mod mirnet_model;
use mirnet_model::MirnetModel;

mod single_file;
use single_file::run as single_file;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mirnet_server",
    about = "A web server for the Low-light image enhancement using mirnet tensorflow model"
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn get_input_bytes(mut payload: Multipart) -> Result<Vec<u8>, actix_web::Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        if field.name() == "input" {
            let mut bytes: Vec<u8> = Vec::new();

            while let Some(chunk) = field.next().await {
                let data = chunk?;
                println!("Writing {} bytes", data.len());
                bytes.write_all(&data)?;
            }

            println!("Found input with {} bytes", bytes.len());

            std::fs::File::create("input.png").unwrap()
                .write_all(&bytes).unwrap();

            return Ok(bytes);
        }
    }

    Err(actix_web::error::ErrorBadRequest("Field not found"))
}

async fn save_file(payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let input_bytes = get_input_bytes(payload).await?;
    let input_image = ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Unable to guess format: {:?}", e)))?
        .decode()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Invalid image: {:?}", e)))?;
    let input_tensor = image_to_tensor(&input_image);
    drop(input_image);
    let model = MirnetModel::new("model").map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Can't initialize model: {:?}", e))
    })?;

    let output_tensor = model.run(&input_tensor).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Error running model: {:?}", e))
    })?;
    let output_image = tensor_to_image(&output_tensor).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Can't convert output: {:?}", e))
    })?;

    let mut output_writerx = Vec::<u8>::new();
    PngEncoder::new(&mut output_writerx).write_image(output_image.as_bytes(),
        output_image.width(),
        output_image.height(),
        image::ColorType::Rgb8).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Can't encode output: {:?}", e))
        })?;

    Ok(HttpResponse::build(StatusCode::OK)
            .content_type("image/png")
            .body(output_writerx))
}

async fn server(port: u16) -> AnyResult<()> {
    println!("Serving on 127.0.0.1:{}", port);

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/run", web::post().to(save_file))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> AnyResult<()> {
    let opt = Opt::from_args();
    if let Some(path) = opt.input {
        single_file(path)?;
    } else {
        server(3001).await?;
    }

    Ok(())
}
