use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;
use validator::Validate;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="user_table")]
pub struct UserTable {
    pub id: i32,
    pub user_name: String,
    pub password: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatUser{
    #[validate(length(min = 3))]
    pub user_name: String,
    #[validate(length(min = 3))]
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct EncodeResponse {
    pub message: String,
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeBody {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProtectedResponse {
    pub id: String,
    pub message: String,   
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,   
}

