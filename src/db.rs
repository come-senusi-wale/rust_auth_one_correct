use crate::models::{UserTable};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;



pub async fn register_user(client: &Client, user_name: String, password: String) -> Result<UserTable, io::Error>{
    let statement = client.prepare("select * from user_table where user_name = $1").await.unwrap();

    let user = client.query(&statement, &[&user_name])
        .await
        .expect("Error getting user")
        .iter()
        .map(|row| UserTable::from_row_ref(row).unwrap())
        .collect::<Vec<UserTable>>();

    if user.len() < 1 {
        let statements = client.prepare("insert into user_table (user_name, password) values ($1, $2) returning id, user_name").await.unwrap();
    
        client.query(&statements, &[&user_name, &password])
            .await
            .expect("Error in registring user")
            .iter()
            .map(|row| UserTable::from_row_ref(row).unwrap())
            .collect::<Vec<UserTable>>()
            .pop()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Error in Rigistering user"))
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


pub async fn login_user(client: &Client, user_name: String, password: String) -> Result<UserTable, io::Error>{
    let statement = client.prepare("select * from user_table where user_name = $1 and password = $2").await.unwrap();
    
    client.query(&statement, &[&user_name, &password])
        .await
        .expect("Error in geting user detail")
        .iter()
        .map(|row| UserTable::from_row_ref(row).unwrap())
        .collect::<Vec<UserTable>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error in user Detail"))
}