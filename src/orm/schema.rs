use diesel::prelude::*;
table! {
    tbl_movement (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}
