use super::schema::products;

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub product_price: i32,
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub product_name: &'a String,
    pub product_price: &'a i32,
}
