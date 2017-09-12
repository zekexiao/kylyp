use utils::schema::{users,message};
use chrono::{DateTime,Utc,NaiveDateTime};

#[derive(Clone,Debug,Serialize,Identifiable,Queryable)]
#[has_many(article,comment)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone,Debug,Serialize,Queryable, Associations)]
#[belongs_to(User)]
pub struct Message {
    pub id: i32,
    pub aid: i32,
    pub cid: i32,
    pub from_uid: i32,
    pub to_uid: i32,
    pub content: String,
    pub mode: i32,
    pub status: i32,
    pub created_at: DateTime<Utc>,
}


#[derive(Insertable)]
#[table_name="message"]
pub struct NewMessage<'a> {
    pub aid: i32,
    pub cid: i32,
    pub from_uid: i32,
    pub to_uid: i32,
    pub content: &'a str,
    pub mode: i32,
    pub status: i32,
    pub created_at: DateTime<Utc>,
}

pub mod message_mode {
    pub const REPLY_ARTICLE: i32 = 1;       // 文章下面回复
    pub const MENTION: i32 = 2;             // 在回复中提到某人
}

pub mod message_status {
    pub const INIT: i32 = 0;                // 初始
    pub const READ: i32 = 1;                // 已读
}    
