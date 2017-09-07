use utils::schema::{article,comment};
use utils::schema::users;

#[derive(Clone,Debug,Serialize,Identifiable,Queryable)]
#[has_many(article,comment)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub regtime: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub regtime: &'a str,
}