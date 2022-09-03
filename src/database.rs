use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


use crate::orm::models::*;
use crate::orm::schema::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_registers() -> Vec<Measure> {
    let connection = &mut establish_connection();
    tbl_measure::table
        .limit(5)
        .load::<Measure>(connection)
        .expect("Error loading measures")
}

