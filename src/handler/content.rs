use diesel;
use diesel::prelude::*;
use model::article::{NewArticle,NewComment};
use model::pg::get_conn;
use model::db::establish_connection;
use chrono::prelude::*;


#[derive(Debug,Serialize)]
pub struct Uarticle {
    pub id: i32,
    pub uid: i32,
    pub category: i32,
    pub status: i32,
    pub comments_count: i32,
    pub title: String,
    pub content: String,
    pub createtime: String,
    pub updatetime: String,
    pub username: String,
}
#[derive(Debug,Serialize)]
pub struct Ucomment {
    pub id: i32,
    pub aid: i32,
    pub uid: i32,
    pub content: String,
    pub createtime: String,
    pub username: String,
}

pub fn article_list() -> Vec<Uarticle> {
    let conn = get_conn();
    let mut article_result: Vec<Uarticle> = vec![];
    for row in &conn.query("SELECT article.*, users.username FROM article, users WHERE article.uid = users.id", &[]).unwrap()
    {
        let result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            content: row.get(6),
            createtime: row.get(7),
            updatetime: row.get(8),
            username: row.get(9),
        };
            article_result.push(result);

    }
    article_result
}


pub fn get_article_by_aid(aid: i32) -> Uarticle {
    let conn = get_conn();
    let mut article_result = Uarticle {
            id: 0,
            uid: 0,
            category: 0,
            status: 0,
            comments_count: 0,
            title: "".to_string(),
            content: "".to_string(),
            createtime: "".to_string(),
            updatetime: "".to_string(),
            username: "".to_string(),
    };
    for row in &conn.query("SELECT article.*, users.username FROM article, users WHERE article.uid = users.id and article.id = $1",&[&aid]).unwrap() {
        article_result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            content: row.get(6),
            createtime: row.get(7),
            updatetime: row.get(8),
            username: row.get(9),
        };
    }
    article_result
}

pub fn get_comment_by_aid(aid: i32) -> Vec<Ucomment> {
    let conn = get_conn();
    let mut result: Vec<Ucomment> = vec![];
    for row in &conn.query("SELECT comment.*, users.username FROM comment, users WHERE comment.uid = users.id and comment.aid = $1 order by comment.id",&[&aid]).unwrap() {
        let comment_result = Ucomment {
            id: row.get(0),
            aid: row.get(1),
            uid: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
            username: row.get(5),
        };
        result.push(comment_result);
    }
    result
    
}

pub fn add_article_by_uid<'a>(uid: i32, title: &'a str, content: &'a str) {
    use utils::schema::article;
    let connection = establish_connection();
    let createtime = &Local::now().to_string();
    let new_article = NewArticle {
        uid : uid,
        title : title,
        content : content,
        createtime : createtime,
    };
    diesel::insert(&new_article).into(article::table).execute(&connection).expect("Error saving new list");
}

pub fn add_comment_by_aid<'a>(aid: i32, uid: i32, content: &'a str) {
    use utils::schema::comment;
    let connection = establish_connection();
    let createtime = &Local::now().to_string();
    let new_comment = NewComment {
        aid: aid,
        uid : uid,
        content : content,
        createtime : createtime,
    };
    diesel::insert(&new_comment).into(comment::table).execute(&connection).expect("Error saving new comment");
}