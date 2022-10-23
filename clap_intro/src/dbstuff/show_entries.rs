use super::super::models::*;
use super::db_conn::establish_connection;
use diesel::prelude::*;
pub fn show_entries(){
    use super::super::schema::entries::dsl::*;
    let connection = &mut establish_connection();
    let results = entries
        .filter(paid.eq(true))
        .limit(5)
        .load::<entries>(connection)
        .expect("Error loading entries");
}


