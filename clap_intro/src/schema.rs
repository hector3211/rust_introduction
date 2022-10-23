diesel::table! {
    // use diesel::sql_types::*;
    // use diesel_full_text_search::types::*;

    entries (id) {
        id -> Int4,
        invoice -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        paid -> Nullable<Bool>,
    }
}
