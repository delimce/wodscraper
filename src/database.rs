use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;

use crate::orm::models::*;
use crate::orm::schema::*;

const DB_POOL_IDLE: Option<u32> = Some(4);
const DB_POOL_MAX: u32 = 10;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

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

pub fn get_register_pool(pool: &MysqlPool) -> Vec<Measure> {
    let use_pool = &mut use_pool(pool);
    tbl_measure::table
        .limit(5)
        .load::<Measure>(use_pool)
        .expect("Error loading measures")
}

pub fn create_pool() -> MysqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .min_idle(DB_POOL_IDLE)
        .max_size(DB_POOL_MAX)
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn use_pool(pool: &MysqlPool) -> MysqlPooledConnection {
    let pool = pool.clone();
    pool.get().expect("Failed to get connection from pool.")
}
