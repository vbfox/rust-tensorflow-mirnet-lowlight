use crate::image_processing::{process_image, run_single_file};
use crate::users::{get_me, login, logout, register, UserDb};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use anyhow::Result as AnyResult;
use rusqlite::Connection;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::{info, instrument};
use tracing_actix_web::TracingLogger;

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

    #[structopt(name = "static", short, long, parse(from_os_str), default_value = "../client/build")]
    static_dir: PathBuf,

    #[structopt(short, long, default_value = "3001")]
    port: u16,
}

#[instrument]
async fn server(port: u16, static_dir: PathBuf) -> AnyResult<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let user_db = UserDb::new(Connection::open("users.db")?);
    user_db.initialize().await?;

    info!("Serving on 127.0.0.1:{}", port);
    info!("Static files will be served from {:?}", &static_dir);

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
            .route("/api/me", web::get().to(get_me))
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::post().to(logout))
            .route("/api/run", web::post().to(process_image))
            .service(actix_files::Files::new("/", &static_dir).show_files_listing().redirect_to_slash_directory().index_file("index.html"))
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
        server(opt.port, opt.static_dir).await?;
    }

    Ok(())
}
