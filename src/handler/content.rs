use diesel;
use diesel::prelude::*;
use model::article::{Article,Comment,NewArticle,NewComment};
use model::pg::get_conn;
use model::db::establish_connection;
use chrono::prelude::*;


#[derive(Debug,Serialize)]
pub struct Uarticle {
    pub id: i32,
    pub uid: i32,
    pub title: String,
    pub content: String,
    pub createtime: String,
    pub username: String,
}

pub fn article_list() -> Vec<Uarticle> {
    let conn = get_conn();
    let mut article_result: Vec<Uarticle> = vec![];
    for row in &conn.query("select * from article inner join users on article.uid = users.id", &[]).unwrap()
    {
        let result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            title: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
            username: row.get(6),
        };
            article_result.push(result);

    }
    article_result
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

pub fn get_article_by_id(pid: i32) -> Article {
    let conn = get_conn();
    let mut result = Article{
        id: 0,
        uid: 0,
        title: "".to_string(),
        content: "".to_string(),
        createtime: "".to_string(),
    };
    for row in &conn.query("select * from article where id = $1 ",&[&pid]).unwrap() {
        result = Article {
            id: row.get(0),
            uid: row.get(1),
            title: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
        };
    }
    result
}

pub fn get_comment_by_pid(pid: i32) -> Vec<Comment> {
    let conn = get_conn();
    let mut result: Vec<Comment> = vec![];
    for row in &conn.query("select * from comment where pid = $1 ",&[&pid]).unwrap() {
        let comment_result = Comment {
            id: row.get(0),
            pid: row.get(1),
            uid: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
        };
        result.push(comment_result);
    }
    result
    
}

pub fn add_comment_by_pid<'a>(pid: i32, uid: i32, content: &'a str) {
    use utils::schema::comment;
    let connection = establish_connection();
    let createtime = &Local::now().to_string();
    let new_comment = NewComment {
        pid: pid,
        uid : uid,
        content : content,
        createtime : createtime,
    };
    diesel::insert(&new_comment).into(comment::table).execute(&connection).expect("Error saving new comment");
}