use actix_web::{error::BlockingError, http, web, HttpResponse, Responder};
use actix_session::Session;
use serde::{Deserialize, Serialize};
use serde_json::json;
use database::user
use error::ToDoError;
use yarte::Template;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool as OtherPool, ConnectionManager};

pub type Pool = OtherPool<ConnectionManager<PgConnection>>;

pub asyns fn send_confirmation(session: Session, data: web::Json<user::RegisterUser>, pool: web::Data<Pool>)
-> Result<HttpResponse, ToDoError> {
    if (is_signed_in(&session)) {
        return Ok(HttpResponse::BadRequest().finish());
    }
    
    let result = web::block(move || create_confirmation(data.into_inner(), &pool)).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => match err {
            BlockingError::Error(auth_error) => Err(auth_error),
            BlockingError::Canceled => Error(ToDoError::GenericError(String::from("Could not complete the process"))),
        },
    }
}

fn create_confirmation(register_user: RegisterUser, pool: &web::Data(Pool)) -> Result<(), AuthError> {
    let confirmation = user::User::create(register_user, pool);

    send_confirmation_mail(&confirmation);
}

