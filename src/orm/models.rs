use crate::orm::schema::tbl_movement;
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
