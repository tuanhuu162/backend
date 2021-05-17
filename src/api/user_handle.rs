use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_session::Session;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::database::user;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool as OtherPool, ConnectionManager};
use std::collections::HashMap;
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct Info {
    name: Option<String>,
}

pub type Pool = OtherPool<ConnectionManager<PgConnection>>;

pub async fn greet(info: web::Query<HashMap<String, String>>) -> impl Responder {
    let default_val = String::from("World");
    let name =  if info.contains_key("name") { String::from(&info["name"]) } else {String::from("World")};
    // println!("{}", &name);
    let output = json!({
        "messenge": format!("Hello {}", &name)
    });
    HttpResponse::Ok().json(output)
}


pub async fn register(pool: web::Data<Pool>, info: web::Json<HashMap<String, String>>) -> impl Responder {
    let user = info.into_inner();
    println!("{:?}", user);
    for key in vec!["name", "email", "password"] {
        if !!!user.contains_key(key) {
            return HttpResponse::BadRequest().json(json!({
                "messenge": "Lack of parameter!!!!!!!!!!!"
            }));
        }
        if key == "name" {
            let re = Regex::new(r"(\w+\s*)+").unwrap();
            if !!!re.is_match(user["name"].to_str()) {
                return HttpResponse::BadRequest().json(json!({
                    "messenge": "Name is contain letter and space only!!!!!!!!!!!"
                }));
            }
        }    
        else if key == "email" {
            let re = Regex::new(r"^([a-z0-9_\.-]+\@([\da-z-]+\.)+[a-z]{2,6})$").unwrap();
            if !!!re.is_match(user["email"].to_str()) {
                return HttpResponse::BadRequest().json(json!({
                    "messenge": "Email format is not right!!!!!!!!!!!"
                }));
            } 
        }
        // } else {
        //     let re = Regex::new(r"^((?=\S*?[A-Z])(?=\S*?[a-z])(?=\S*?[0-9])(?=\S*?[!@#$%^&*()_+\-=?><]).{6,})\S$").unwrap();
        //     if !!!re.is_match(user["hash"]) {
        //         return HttpResponse::BadRequest().json(json!({
        //             "messenge": "Name is contain letter and space only!!!!!!!!!!!"
        //         }));
        //     } 
        // }

    }
    let register_user = user::RegisterUser {
        email: String::from(&user["email"]),
        name: String::from(&user["name"]),
        hash: String::from(&user["password"])
    };
    let email = String::from(&user["email"]);
    // println!("email {:?}", &email);
    let result = user::User::create(&register_user, &pool);
    // println!("result {:?}", &result);
    match result {
        Ok(_) =>     HttpResponse::Ok().json(json!({
            "messenge": "Successful register user!!!!!!"
        })),
        Err(err) => HttpResponse::BadRequest().json(json!({
                "messenge": err
        })),
    }
}


pub async fn login(pool: web::Data<Pool>, session: Session, req: HttpRequest, info: web::Json<HashMap<String, String>>) -> impl Responder {
    let auth = info.into_inner();
    println!("{:?}", auth);
    for key in vec!["email", "hash"] {
        if !!! auth.contains_key(key){
            return HttpResponse::BadRequest().json(json!({
                "messenge": "Lack of parameter!!!!!!!!!!!"
            }));
        }
    }
    let auth_user = user::AuthUser {
        email: auth["email"],
        password: auth["password"]
    };
    let session_user = auth_user.login(&pool)?;
    match session_user {
        Ok(_) => HttpResponse::Ok().json(json!({
            "messenge": "Successful login!!"
        })),
        Err(err) => HttpResponse::BadRequest().json(json!({
            "messenge": err
        }))
    }
}
