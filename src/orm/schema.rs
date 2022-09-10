use diesel::prelude::*;
table! {
    tbl_movement (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

table! {
    tbl_wod (id) {
        id -> Integer,
        name -> Varchar,
        type_id -> Integer,
        category_id -> Integer,
        rounds -> Integer,
        timecap -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}
