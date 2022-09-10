use crate::orm::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = tbl_movement)]
pub struct Movement {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name = "tbl_movement"]
pub struct NewMovement {
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name = "tbl_wod"]
pub struct NewWod {
    pub name: String,
    pub type_id: i32,
    pub category_id: i32,
    pub rounds: i32,
    pub timecap: String,
    pub created_at: String,
    pub updated_at: String,
}
