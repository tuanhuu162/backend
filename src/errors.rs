use std::fmt;
use argon2;
use diesel::result::{Error as DBErrorDefault};
use std::convert::From;
use actix_web::{body::Body, BaseHttpResponse, HttpResponse, ResponseError};
use uuid::Error as UuidError;
use serde_json::json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ToDoError {
    DuplicateValue(String),
    BadId,
    GenericError(String),
    #[serde(skip)]
    HashError(argon2::Error),
    #[serde(skip)]
    DBError(DBErrorDefault),
    AuthenticationError(String),
    PasswordNotMatch(String),
    WrongPassword(String)
}

impl From<argon2::Error> for ToDoError {
    fn from(error: argon2::Error) -> Self {
        ToDoError::HashError(error)
    }
}

impl From<UuidError> for ToDoError {
    fn from(error: UuidError) -> Self {
        println!("{:?}", error);
        ToDoError::BadId
    }
}

impl From<DBErrorDefault> for ToDoError {
    fn from(error: DBErrorDefault) -> Self {
        ToDoError::DBError(error)
    }
}

impl fmt::Display for ToDoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToDoError::HashError(error) => write!(f, "{}", error),
            ToDoError::DBError(error) => write!(f, "{}", error),
            ToDoError::PasswordNotMatch(error) => write!(f, "{}", error),
            ToDoError::WrongPassword(error) => write!(f, "{}", error),
            ToDoError::BadId => write!(f, "Invalid ID"),
            ToDoError::DuplicateValue(error) => write!(f, "{}", error),
            ToDoError::AuthenticationError(error) => write!(f, "{}", error),
            ToDoError::GenericError(error) => write!(f, "{}", error),
        }
    }
}

impl ResponseError for ToDoError {
    fn error_response(&self) -> BaseHttpResponse<Body> {
        match self {
            ToDoError::HashError(error) => {
                println!("{:?}", error);
                BaseHttpResponse::from(HttpResponse::BadRequest().json(json!({
                "messenge": "Hashing error"
                })))
            },
            ToDoError::DBError(error) => {
                println!("{:?}", error);
                BaseHttpResponse::from(HttpResponse::BadRequest().json(json!({
                    "messenge": "There is some error with databases"
                })))
            },
            ToDoError::PasswordNotMatch(error) => BaseHttpResponse::from(HttpResponse::BadRequest().json(json!({
                "messenge": error
            }))),
            ToDoError::WrongPassword(error) => BaseHttpResponse::from(HttpResponse::BadRequest().json(json!({
                "messenge": error
            }))),
            ToDoError::BadId => BaseHttpResponse::from(HttpResponse::BadRequest().json(json!({
                "messenge": "Invalid ID"
            }))),
            ToDoError::DuplicateValue(error) => BaseHttpResponse::from(HttpResponse::BadRequest().json(json!({
                "messenge": error
            }))),
            ToDoError::AuthenticationError(error) => BaseHttpResponse::from(HttpResponse::Unauthorized().json(json!({
                "messenge": error
            }))),
            ToDoError::GenericError(error) => BaseHttpResponse::from(HttpResponse::BadRequest().json(json!({
                "messenge": error
            }))),
        }
    }
}
