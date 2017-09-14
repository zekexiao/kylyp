use model::user::User;
use utils::schema::{article,comment};
use chrono::{DateTime,Utc};

#[derive(Clone,Debug,Serialize,Queryable, Associations)]
#[belongs_to(User)]
pub struct Article {
    pub id: i32,
    pub uid: i32,
    pub category: String,
    pub status: i32,
    pub comments_count: i32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Insertable)]
#[table_name="article"]
pub struct NewArticle<'a> {
    pub uid: i32,
    pub category: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone,Debug,Serialize,Queryable,  Associations)]
#[belongs_to(User)]
pub struct Comment {
    pub id: i32,
    pub aid: i32,
    pub uid: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="comment"]
pub struct NewComment<'a> {
    pub aid: i32,
    pub uid: i32,
    pub content: &'a str,
    pub created_at: DateTime<Utc>,
}

pub mod STATUS {
    pub const NORMAL: i32 = 0;
    pub const DELETED: i32 = -1;
}
