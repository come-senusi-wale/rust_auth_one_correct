use actix_web::{web, Responder, HttpResponse,};

use crate::db;
use crate::models::{Status, CreatUser, EncodeResponse, Claims, ProtectedResponse, DecodeBody, Response};

//use std::io::ErrorKind::Other;
use deadpool_postgres::{Pool, Client};

use chrono::{Utc, Duration};
use jsonwebtoken::{
    encode,
    decode,
    Header,
    EncodingKey, 
    DecodingKey,
    Validation,
    Algorithm,
    TokenData,
    errors::Error as JWTError,
};
pub struct AppState{
    pub db: Pool,
    pub secret: String
}

pub async fn status() -> impl Responder{
    HttpResponse::Ok()
     .json( Status {status: "up".to_string()})
}

pub async fn register_user(db_pool: web::Data<AppState>, json: web::Json<CreatUser>) ->impl Responder{
    let client: Client = db_pool.db.get().await.expect("Error connecting to the database");

    let result = db::register_user(&client, json.user_name.clone(), json.password.clone()).await;

    // match result {
    //     Ok(user) => HttpResponse::Ok().json(user),
    //     Err(_) => HttpResponse::InternalServerError().into()
    // }

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().json(Response {message: e.to_string()})
    }
}

pub async fn get_user(db_pool: web::Data<AppState>) ->impl Responder{
    let client: Client = db_pool.db.get().await.expect("Error connecting to the database");

    let result = db::get_user(&client).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user), 
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn login_user(app_data: web::Data<AppState>, json: web::Json<CreatUser>) ->impl Responder{
    let client: Client = app_data.db.get().await.expect("Error connecting to the database");

    let result = db::login_user(&client, json.user_name.clone(), json.password.clone()).await;

    match result {
        Ok(user) => {
            //HttpResponse::Ok().json(user)

            // get the user id
            let id: usize = user.id.try_into().unwrap();

            // set the token expiration to one day
            let exp: usize = (Utc::now() + Duration::days(1)).timestamp() as usize;

            let claims: Claims = Claims{id, exp};

            let token: String = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(app_data.secret.as_str().as_ref())
            ).unwrap();

            HttpResponse::Ok().json(EncodeResponse{
                message: "success".to_owned(),
                token,
            })
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}



pub async fn protected(body: web::Json<DecodeBody>, app_data: web::Data<AppState>,) -> HttpResponse{

    let decode: Result<TokenData<Claims>, JWTError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(app_data.secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256)
    );

    match decode {
        Ok(token)=>HttpResponse::Ok().json( ProtectedResponse{
            id: token.claims.id.to_string(),
            message: "Authorize".to_string(),
        }),
        Err(e)=> HttpResponse::BadRequest().json(Response {message: e.to_string()})
    }
    //HttpResponse::Ok().json(Response{message: "decode_token".to_owned()})
}


// pub async fn protected(auth_token: AuthenticationToken) -> HttpResponse{
//     println!("{}", auth_token.id);

//     //HttpResponse::Ok().json(Response{message: "protected".to_owned()})
//     HttpResponse::Ok().json(ProtectedResponse{
//         id: auth_token.id.to_string(),
//         message: "you can continue access this page".to_string(), 
//     })
// } 
