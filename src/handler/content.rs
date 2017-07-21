use diesel;
use diesel::prelude::*;
use model::db::establish_connection;
use std::fmt::Debug;
use model::user::{User,NewUser};
use model::container::{List,NewList,NewReply};
use model::pg::get_conn;

#[derive(Debug,Serialize)]
pub struct Ulist {
    pub id: i32,
    pub uid: i32,
    pub title: String,
    pub content: String,
    pub createtime: String,
    pub username: String,
}

pub fn date_list() -> Vec<Ulist> {
    use utils::schema::users::dsl::{users,id};
    use utils::schema::list::dsl::{list,uid};
    use utils::schema::reply::dsl::{reply,pid};
    let conn = get_conn();
    let mut list_result: Vec<Ulist> = vec![];
    for row in &conn.query("select list.*, username from list, users where list.uid = users.id", &[]).unwrap()
    {
           let result = Ulist {
            id: row.get(0),
            uid: row.get(1),
            title: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
            username: row.get(5),
        };
            list_result.push(result);

    }
    list_result
}