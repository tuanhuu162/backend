use actix_session::Session;
use crate::database::user::SessionUser;
use crate::errors::ToDoError;
use serde_json;

pub fn is_signed_in(session: &Session) -> bool {
    match get_current_user(&session) {
        Ok(_) => true,
        _ => false
    }
}



fn get_current_user(session: &Session) -> Result<SessionUser, ToDoError>{
    let msg = "Could not retrieve user from session";

    session::get::<String>("user")
        .map_err(|_| ToDoError::AuthenticationError(String::from(msg)))
        .unwrap()
        .map_or(
            Err(ToDoError::AuthenticationError(String::from(msg))),
            |user| serde_json::from_str(&user).or_else(|_| Err(ToDoError::AuthenticationError(String::from(msg))))
        )
}

pub fn set_current_user(user: &SessionUser,session: &Session) -> () {
    session.set("user", serde_json::to_string(user).unwrap()).unwrap();
}

pub fn to_home() -> HttpResponse {
    HttpResponse::Found().header(LOCATOIN, "/me").finish()
}

