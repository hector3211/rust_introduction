
diesel::table! {
    entries (id) {
        id -> Int4,
        invoice -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        paid -> Nullable<Bool>,
    }
}


