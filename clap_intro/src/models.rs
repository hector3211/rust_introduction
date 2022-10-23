use diesel::prelude::*;
use crate::schema::entries;
#[derive(Queryable)]
pub struct Entry {
    pub id:i32,
    pub invoice: i32,
    pub name: String,
    pub paid: bool,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntries<'a> {
    pub name: &'a str,
    pub invoice: &'a i32,
    pub paid: &'a bool,
}

