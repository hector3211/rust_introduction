use diesel::prelude::*;
use crate::schema::entries;
#[derive(Queryable)]
pub struct Entry {
    pub id:u32,
    pub invoice: u32,
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

