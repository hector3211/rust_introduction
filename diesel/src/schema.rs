table! {
    videos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        removed -> Bool,
        created_at -> Timestamp,
    }
}
