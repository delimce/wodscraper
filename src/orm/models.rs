use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = tbl_measure)]
pub struct Measure {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
