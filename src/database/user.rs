use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use argon2::{self, Config};
use uuid::Uuid;
use actix_web::{web};

// use diesel::prelude::{Queryable, Insertable};
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool as OtherPool, ConnectionManager};

use crate::vars;
use crate::schema::users;
use crate::errors::ToDoError;

pub type Pool = OtherPool<ConnectionManager<PgConnection>>;


#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)]
    pub user_id: i32,
    #[serde(skip)]
    pub user_uuid: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip)]
    pub hash: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub hash: String
}


#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    // pub user_id: Option<String>,
    pub email: String, 
    pub name: String,
    pub hash: String
}


impl User {
    pub fn hash_password(plain: String) -> Result<String, ToDoError> {
        let salt = vars::secret_key();
        let config = Config::default();
        let hash = argon2::hash_encoded(plain.as_bytes(), salt.as_bytes(), &config).unwrap();
        Ok(hash)
    }
    pub fn create(register_user: &RegisterUser, connection: &web::Data<Pool>) -> Result<User, ToDoError>{
        use diesel::RunQueryDsl;
        
        let result = diesel::insert_into(users::table)
                .values(NewUser {
                    email: register_user.email.to_string(),
                    name: register_user.name.to_string(),
                    hash: Self::hash_password(register_user.hash.to_string())?
                })
                .get_result(&connection.get().unwrap())?;
        Ok(result)
    }
}


#[derive(Debug, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub user_id: u32,
    pub email: String 
}

impl AuthUser{
    pub fn login(&self, connection: &web::Data<Pool>) -> Result<SessionUser, ToDoError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::users::dsl::email;

        let mut records = 
            users::table
                .filter(email.eq(&self.email))
                .load::<SessionUser>(&connection.get().unwrap())?;
        let user = records
            .pop()
            .ok_or(ToDoError::DBError(diesel::result::Error::NotFound))?;
        
        let verify_password = argon2::verify_encoded(&self.password, &user.hash.as_bytes())
            .map_err(|_error| {
                ToDoError::WrongPassword(
                    "Wrong password, check again please".to_string()
                )
            })?;

        if verify_password {
            Ok(SessionUser)
        } else {
            Err(ToDoError::WrongPassword({
                "Wrong password, check again please".to_string()
            }))
        }
    }
}

impl From<User> for SessionUser {
    fn from(User{user_id, email, ..} ) -> SessionUser {
        SessionUser { user_id, email }
    }
}

impl From<User> for RegisterUser {
    fn from(User{email, name, hash, ..}) -> RegisterUser {
        RegisterUser { email, name, hash }
    }
}


