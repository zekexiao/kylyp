use diesel;
use diesel::prelude::*;
use diesel::prelude::JoinDsl;
use std::fmt::Debug;
use model::user::{User,NewUser};
use model::container::{List,Reply,NewList,NewReply};
use model::pg::get_conn;
use model::db::establish_connection;
use controller::user::UserOr;
use chrono::prelude::*;


#[derive(Debug,Serialize)]
pub struct Ulist {
    pub id: i32,
    pub uid: i32,
    pub title: String,
    pub content: String,
    pub createtime: String,
    pub username: String,
}

pub fn date_index() -> Vec<Ulist> {
    let conn = get_conn();
    let mut list_result: Vec<Ulist> = vec![];
    for row in &conn.query("select * from list inner join users on list.uid = users.id", &[]).unwrap()
    {
        let result = Ulist {
            id: row.get(0),
            uid: row.get(1),
            title: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
            username: row.get(6),
        };
            list_result.push(result);

    }
    list_result
}

// pub fn date_index() -> Vec<Ulist> {
//     use utils::schema::list;
//     use utils::schema::users;
//     let connection = establish_connection();
//     let mut list_result: Vec<Ulist> = vec![];
//     let data = list::table.inner_join(users::table).load(&connection);
//     println!("================{:?}===========",data );
//     // for row in data {
//     //     let result = Ulist {
//     //         id: row.0,
//     //         uid: row.1,
//     //         title: row.2,
//     //         content: row.3,
//     //         createtime: row.4,
//     //         username: row.6,
//     //     };
//     //     list_result.push(result);
//     // }
//     list_result
// }

pub fn add_topic_uid<'a>(uid: i32, title: &'a str, content: &'a str) {
    use utils::schema::list;
    let connection = establish_connection();
    let createtime = &Local::now().to_string();
    let new_list = NewList {
        uid : uid,
        title : title,
        content : content,
        createtime : createtime,
    };
    diesel::insert(&new_list).into(list::table).execute(&connection).expect("Error saving new list");
}

pub fn get_list_by_id(pid: i32) -> List {
    let conn = get_conn();
    let mut result = List{
        id: 0,
        uid: 0,
        title: "".to_string(),
        content: "".to_string(),
        createtime: "".to_string(),
    };
    for row in &conn.query("select * from list where id = $1 ",&[&pid]).unwrap() {
        result = List {
            id: row.get(0),
            uid: row.get(1),
            title: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
        };
    }
    result
}

pub fn get_reply_by_pid(pid: i32) -> Vec<Reply> {
    let conn = get_conn();
    let mut result: Vec<Reply> = vec![];
    for row in &conn.query("select * from reply where pid = $1 ",&[&pid]).unwrap() {
        let reply_result = Reply {
            id: row.get(0),
            pid: row.get(1),
            uid: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
        };
        result.push(reply_result);
    }
    result
    
}

pub fn add_reply_pid<'a>(pid: i32, uid: i32, content: &'a str) {
    use utils::schema::reply;
    let connection = establish_connection();
    let createtime = &Local::now().to_string();
    let new_reply = NewReply {
        pid: pid,
        uid : uid,
        content : content,
        createtime : createtime,
    };
    diesel::insert(&new_reply).into(reply::table).execute(&connection).expect("Error saving new reply");
}