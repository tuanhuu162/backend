#[macro_use]
extern crate diesel;
extern crate log;

pub mod api;
pub(crate) mod database;
mod jwt;
// mod utils;
pub mod schema;
pub mod vars;
pub mod errors;
pub mod cli;
