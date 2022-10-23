use diesel::prelude::*;

#[derive(Queryable)]
pub struct Entry {
    pub id:u32,
    pub invoice: u32,
    pub name: String,
    pub paid: bool,
}
