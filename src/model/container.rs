use utils::schema::{list,reply};

#[derive(Clone,Debug)]
#[derive(Queryable, Associations)]
#[belongs_to(User)]
pub struct List {
    pub id: i32,
    pub uid: i32,
    pub title: String,
    pub content: String,
    pub createtime: String,
}


#[derive(Insertable)]
#[table_name="list"]
pub struct NewList<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub createtime: &'a str,
}

#[derive(Clone,Debug)]
#[derive(Queryable,  Associations)]
#[belongs_to(User)]
pub struct Reply {
    pub id: i32,
    pub pid: i32,
    pub uid: i32,
    pub content: String,
    pub createtime: String,
}

#[derive(Insertable)]
#[table_name="reply"]
pub struct NewReply<'a> {
    pub content: &'a str,
    pub createtime: &'a str,
}
