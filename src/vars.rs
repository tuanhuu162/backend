use dotenv::dotenv;

use std::env::var;

pub fn database_url() -> String {
    dotenv().ok();
    var("DATABASE_URL").expect("DATABASE_URL is not set")
}


pub fn secret_key() -> String {
    dotenv().ok();

    var("SECRET_KEY").unwrap_or("secret_is_a_secret".repeat(3))
}

pub fn host() -> String {
    dotenv().ok();

    var("HOST").unwrap_or(String::from("localhost"))
}

pub fn port() -> u16 {
    dotenv().ok();

    var("PORT").expect("PORT is not set").parse::<u16>().ok().expect("PORT should be an integer")
}

pub fn smtp_url() -> String {
    dotenv().ok();

    var("SMTP_USERNAME").expect("SMTP_USERNAME is not set")
}

pub fn smtp_password() -> String {
    dotenv().ok();

    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set")
}

pub fn smtp_host() -> String {
    dotenv().ok();

    var("SMTP_HOST").expect("SMTP_HOST is not set")
}

pub fn smtp_port() -> u16 {
    dotenv().ok();

    var("SMTP_PORT").expect("SMTP_PORT is not set").parse::<u16>().ok()
        .expect("SMTP_PORT should be an integer")
}

#[allow(dead_code)]
pub fn smtp_sender_name() -> String {
    dotenv().ok();

    var("SMTP_SENDER_NAME").expect("SMTP_SENDER_NAME is not set")
}