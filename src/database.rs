use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;
use chrono::prelude::*;

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

pub fn get_registers() -> Vec<Movement> {
    let connection = &mut establish_connection();
    tbl_movement::table
        .limit(5)
        .load::<Movement>(connection)
        .expect("Error loading Movements")
}

pub fn get_register_pool(pool: &MysqlPool) -> Vec<Movement> {
    let use_pool = &mut use_pool(pool);
    tbl_movement::table
        .limit(5)
        .load::<Movement>(use_pool)
        .expect("Error loading Movements")
}

pub fn insert_movements(pool: &MysqlPool, movements: Vec<String>) {
    let use_pool = &mut use_pool(pool);
    let mut movements_to_insert = Vec::new();
    for movement in movements {
        let new_movement = NewMovement {
            name: movement,
            created_at: get_current_time(),
            updated_at: get_current_time(),
        };
        movements_to_insert.push(new_movement);
    }
    diesel::insert_into(tbl_movement::table)
        .values(&movements_to_insert)
        .execute(use_pool)
        .expect("Error saving new movement");
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

fn get_current_time() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.format("%Y-%m-%d %H:%M:%S").to_string()
}