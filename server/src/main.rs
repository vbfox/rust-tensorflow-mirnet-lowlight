use crate::image_processing::{process_image, run_single_file};
use crate::users::{login, logout, register, UserDb};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;
use anyhow::Result as AnyResult;
use rusqlite::Connection;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::{info, instrument};

// https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
// https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

mod image_processing;
mod users;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mirnet_server",
    about = "A web server for the Low-light image enhancement using mirnet tensorflow model"
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

#[instrument]
async fn server(port: u16) -> AnyResult<()> {
    tracing_subscriber::fmt::init();

    let user_db = UserDb::new(Connection::open("users.db")?);
    user_db.initialize().await?;

    info!("Serving on 127.0.0.1:{}", port);

    let user_db = web::Data::new(user_db);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .wrap(IdentityService::new(
                // All-zero key used as we only store unique session IDs for now
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .route("/api/register", web::post().to(register))
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::post().to(logout))
            .route("/api/run", web::post().to(process_image))
            .app_data(user_db.clone())
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
