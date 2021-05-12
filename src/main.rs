#[macro_use]
extern crate diesel;
extern crate log;

use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager};
// use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use std::io::Result;


pub mod api;
mod database;
mod schema;
mod errors;
mod cli;
mod vars;


#[actix_web::main]
async fn main() -> Result<()> {

    // Initiates error logger
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    // Sets options to environment variables
    let opt = {
        use structopt::StructOpt;
        cli::Opt::from_args()
    };

    // Server port
    let port = opt.port;

    // Server host
    let host = opt.host.as_str();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // .wrap(
            //     Cors::default()
            //     .allowed_origin("*")
            // )
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(api::user_handle::greet))
            .route("/health_check", web::get().to(api::health_check))
            .route("/register", web::post().to(api::user_handle::register))
    })
    .bind((host, port))
    .unwrap()
    // Start server
    .run();

    eprintln!("Listening on {}:{}", host, port);

    server.await
}
