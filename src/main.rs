use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use anyhow::Context;
use anyhow::Result as AnyResult;
use image::io::Reader as ImageReader;
use image::GenericImageView;
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

async fn server() -> AnyResult<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
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
        server().await?;
    }

    Ok(())
}
