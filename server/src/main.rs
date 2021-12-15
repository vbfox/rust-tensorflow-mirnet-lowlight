use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result as AnyResult;
use image_processing::{process_image, run_single_file};
use std::path::PathBuf;
use structopt::StructOpt;

// https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
// https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

mod image_processing;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mirnet_server",
    about = "A web server for the Low-light image enhancement using mirnet tensorflow model"
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

async fn create_account(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn login(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn server(port: u16) -> AnyResult<()> {
    println!("Serving on 127.0.0.1:{}", port);

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .route("/create", web::get().to(create_account))
            .route("/login", web::get().to(login))
            .route("/run", web::post().to(process_image))
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
        run_single_file(path, "out.png")?;
    } else {
        server(3001).await?;
    }

    Ok(())
}
