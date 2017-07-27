use diesel;
use diesel::prelude::*;
use std::fmt::Debug;
use model::user::{User,NewUser};
use model::container::{List,NewList,NewReply};
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


pub fn get_add_topic<'a>(uid: i32, title: &'a str, content: &'a str) {
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
    
    // let mut list_result: Vec<Ulist> = vec![];
    // let connection = establish_connection();
    // let result =  list.filter(&pid.eq(&pid.id)).load::<List>(&connection);
    // let data_result = match result {
    //     Ok(result_first) => match result_first.first() {
    //         Some(data) => Some(data.clone()),
    //         None => None
    //     },
    //     Err(_) => None
    // };
    // for row in &conn.query("select * from list where id = &pid.id",&[]).unwrap() {
    //      let result = Ulist {
    //         id: row.get(0),
    //         uid: row.get(1),
    //         title: row.get(2),
    //         content: row.get(3),
    //         createtime: row.get(4),
    //         username: row.get(5),
    //     };
    //         list_result.push(result);
    // }
    // data_result
}

// pub fn get_reply(id: &str) {
//     let conn = get_conn();
//     let a_list = conn.query("select * from list where id = $1",&[id]).unwrap();
    
// }

pub fn add_reply_by_id() {

}