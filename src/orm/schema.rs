use diesel::prelude::*;
table! {
    tbl_measure (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}
