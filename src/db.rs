use crate::models::{UserTable};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;
use argonautica::{Hasher,};


pub async fn register_user(client: &Client, user_name: String, password: String) -> Result<(), io::Error>{

    if user_name.len() < 1 || password.len() < 1 {
        return Err(io::Error::new(io::ErrorKind::Other, "user name or password must not be empty"));
    }

    if password.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::Other, "password must be more than three character"));
    }

    let statement = client.prepare("select * from user_table where user_name = $1").await.unwrap();

    let user = client.query(&statement, &[&user_name])
        .await
        .expect("Error getting user")
        .iter()
        .map(|row| UserTable::from_row_ref(row).unwrap())
        .collect::<Vec<UserTable>>();

    if user.len() < 1 {
        let hash_secret = std::env::var("HASH_SECRET").expect("HASH Secret must be set");
        let mut hasher = Hasher::default();

        let hash = hasher
            .with_password(password)
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();

        let statements = client.prepare("insert into user_table (user_name, password) values ($1, $2) ").await.unwrap();
    
        let result = client.execute(&statements, &[&user_name, &hash])
            .await
            .expect("Error in registring user");
            // .iter()
            // .map(|row| UserTable::from_row_ref(row).unwrap())
            // .collect::<Vec<UserTable>>()
            // .pop()
            // .ok_or(io::Error::new(io::ErrorKind::Other, "Error in Rigistering user"))
            match result {
                ref created if *created == 1 => Ok(()),
                _=> Err(io::Error::new(io::ErrorKind::Other, "faild to register user"))
            }

    }else {
        Err(io::Error::new(io::ErrorKind::Other, "user name already exis"))
    }

    
}

pub async fn get_user(client: &Client) -> Result<Vec<UserTable>, io::Error> {

    let statement = client.prepare("select * from user_table").await.unwrap();

    let user = client.query(&statement, &[])
        .await
        .expect("Error getting todo")
        .iter()
        .map(|row| UserTable::from_row_ref(row).unwrap())
        .collect::<Vec<UserTable>>();

    Ok(user)

}


pub async fn login_user(client: &Client, user_name: String) -> Result<UserTable, io::Error>{
    let statement = client.prepare("select * from user_table where user_name = $1").await.unwrap();
    
    client.query(&statement, &[&user_name,])
        .await
        .expect("Error in geting user detail")
        .iter()
        .map(|row| UserTable::from_row_ref(row).unwrap())
        .collect::<Vec<UserTable>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error in user Detail"))
}