use diesel;
use diesel::prelude::*;
use model::article::{Article,Comment,NewArticle,NewComment,STATUS};
use model::user::{User,NewMessage,message_mode,message_status};
use model::pg::get_conn;
use model::db::establish_connection;
use controller::user::UserId;
use chrono::prelude::*;
use regex::{Regex,Captures};
use config::*;
use CFG_DEFAULT;
use chrono::{DateTime,Utc};
use spongedown;

#[derive(Debug, Serialize)]
pub struct Uarticle {
    pub id: i32,
    pub uid: i32,
    pub category: String,
    pub status: i32,
    pub comments_count: i32,
    pub title: String,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct Ucomment {
    pub id: i32,
    pub aid: i32,
    pub uid: i32,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub username: String,
}
#[derive(Debug,Serialize)]
pub struct UserComment {
    pub id: i32,
    pub uid: i32,
    pub category: String,
    pub status: i32,
    pub comments_count: i32,
    pub title: String,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub comment_id: i32,
    pub comment_aid: i32,
    pub comment_uid: i32,
    pub comment_content: String,
    pub comment_createtime: DateTime<Utc>,
}
#[derive(Debug,Serialize)]
pub struct UserMessage {
    pub message_status: i32,
    pub message_createtime: DateTime<Utc>,
    pub comment_raw: String,
    pub comment_cooked: String,
    pub from_uid: i32,
    pub from_uid_name: String,
    pub from_uid_email: String,
    pub article_id: i32,
    pub article_title: String,
}

#[derive(Debug, Deserialize)]
struct Setting {
    development: Development,
}
#[derive(Debug, Deserialize)]
struct Development {
    address: Option<String>,
    port: Option<String>,
}
struct CommentId{
    id: i32,
}
struct ToUid{
    id: i32,
}

impl Setting {
    pub fn new(& mut self) -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(File::with_name(CFG_DEFAULT))?;
        self.development.address = cfg.get("development.address").ok();
        self.development.port = cfg.get("development.port").ok();
        cfg.try_into()
    }
}


pub fn article_list() -> Vec<Uarticle> {
    let conn = get_conn();
    let mut article_result: Vec<Uarticle> = vec![];
    for row in &conn.query("SELECT article.id, article.uid, article.category, article.status, article.comments_count, article.title, article.raw,
                           article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id ORDER BY article.id DESC", &[]).unwrap()
    {
        let result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            updated_at: row.get(9),
            username: row.get(10),
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
        category: "".to_string(),
        status: 0,
        comments_count: 0,
        title: "".to_string(),
        raw: "".to_string(),
        cooked: "".to_string(),
        created_at: Utc::now(), 
        updated_at: Utc::now(), 
        username: "".to_string(),
    };
    for row in &conn.query("SELECT article.id, article.uid, article.category, article.status,
                            article.comments_count, article.title, article.raw, article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND article.id = $1", &[&aid]).unwrap() {
        article_result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            updated_at: row.get(9),
            username: row.get(10),
        };
    }
    article_result
}

pub fn get_comment_by_aid(aid: i32) -> Vec<Ucomment> {
    let conn = get_conn();
    let mut result: Vec<Ucomment> = vec![];
    for row in &conn.query("SELECT comment.id, comment.aid, comment.uid, comment.raw, comment.cooked, comment.created_at, users.username 
                            FROM comment, users WHERE comment.uid = users.id AND comment.aid = $1 ORDER BY comment.id", &[&aid]).unwrap() {
        let mut comment_result = Ucomment {
            id: row.get(0),
            aid: row.get(1),
            uid: row.get(2),
            raw: row.get(3),
            cooked: row.get(4),
            created_at: row.get(5),
            username: row.get(6),
        };
        result.push(comment_result);
    }
    result
}

pub fn add_article_by_uid<'a>(uid: i32, category: &'a str, title: &'a str, raw: &'a str) {
    use utils::schema::article;
    let connection = establish_connection();
    let created_at = Utc::now();
    let updated_at = Utc::now();
    let new_article = NewArticle {
        uid: uid,
        category: category,
        title: title,
        raw: raw,
        cooked: &spongedown::parse(&raw),
        created_at: created_at,
        updated_at: updated_at,
    };
    diesel::insert(&new_article).into(article::table).execute(&connection).expect("Error saving new list");
}
        
pub fn add_comment_by_aid<'a>(aid: i32, uid: i32, raw: &'a str,) {
    let env = Development {
        address: Some("".to_string()),
        port: Some("".to_string()),
    };
    let mut path =  Setting { development: env};
    let f_path = Setting::new(& mut path).unwrap();
    let mut forum_path = "".to_string();
    {
        if let Some(address) = f_path.development.address { 
            forum_path =  address;
        };
    }
    let mut app_path = "http://".to_string() + &forum_path + &":".to_string();
    {
        if let Some(port) = f_path.development.port { 
            app_path +=  &port;
        };
    }

    let conn = get_conn();
    use utils::schema::comment;
    let connection = establish_connection();
    let re = Regex::new(r"\B@([\da-zA-Z_]+)").unwrap();
    let mut to_uids: Vec<i32> = Vec::new();
    let new_content = re.replace_all(&raw, |cap: &Captures| {
        match get_uids(cap.at(1).unwrap()) {
            Some(user_id) => {
                to_uids.push(user_id);
                format!("[@{}]({}{}{})",
                        cap.at(1).unwrap(),
                        app_path,
                        "/user/",
                        user_id)
            },
            None => format!("@{}", cap.at(1).unwrap()),
        }
    });
    let created_at = Utc::now();
    let new_comment = NewComment {
        aid : aid,
        uid : uid,
        raw : &new_content,
        cooked: &spongedown::parse(&new_content),
        created_at : created_at,
    };
    diesel::insert(&new_comment).into(comment::table).execute(&connection).expect("Error saving new comment");
    
    let mut comment_id: i32 = 0;
    for row in &conn.query("SELECT comment.id FROM comment WHERE comment.raw = $1",&[&raw]).unwrap() {
        let comment = CommentId {
            id: row.get(0),
        };
        comment_id = comment.id;
    }
    conn.execute("UPDATE article SET comments_count = comments_count + 1 WHERE id = $1", &[&aid]).unwrap();
    let mut author_id: i32 = 0;
    for row in &conn.query("SELECT article.uid FROM article WHERE article.id = $1",&[&aid]).unwrap() {
        let t_uid = ToUid {
            id: row.get(0),
        };
        author_id = t_uid.id;
    }
    if uid != author_id {
        conn.execute("INSERT INTO message (aid, cid, from_uid, to_uid, raw, mode, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                 &[&aid, &comment_id, &uid, &author_id, &raw, &message_mode::REPLY_ARTICLE, &message_status::INIT, &created_at]).unwrap();
    }
    to_uids.sort();
    to_uids.dedup();
    for to_uid in to_uids.iter().filter(|&x| *x != author_id && *x != uid) {
        conn.execute("INSERT INTO message(aid, cid, from_uid, to_uid, raw, mode, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&aid, &comment_id, &uid, &to_uid, &raw, &message_mode::REPLY_ARTICLE, &message_status::INIT, &created_at]).unwrap();
    }
}

pub fn get_uids(username: &str) -> Option<i32> {

    let conn = get_conn();
    let mut to_uid: Option<i32> = Some(0);
    for row in &conn.query("SELECT id FROM user WHERE username = $1",&[&username]).unwrap() {
        let uid = ToUid {
            id: row.get(0),
        };
        to_uid = Some(uid.id);
    }
    to_uid
}

pub fn get_user_info(user_id: i32) -> Option<User> {
    use utils::schema::users::dsl::*;
    let connection = establish_connection();
    let uid = user_id;
    let user_result =  users.filter(&id.eq(&uid)).load::<User>(&connection);
    let login_user = match user_result {
        Ok(user_s) => match user_s.first() {
            Some(a_user) => Some(a_user.clone()),
            None => None,
        },
        Err(_) => None,
    };
    login_user
}

pub fn get_user_articles(user_id: i32) -> Vec<Article> {
    let conn = get_conn();
    let u_id = user_id;
    let mut user_articles: Vec<Article> = vec![];
    for row in &conn.query("SELECT article.id, article.uid, article.category, article.status, 
                            article.comments_count, article.title, article.raw, article.cooked, article.created_at, article.updated_at 
                           FROM article WHERE article.uid = $1 ",&[&u_id]).unwrap() {
        let article = Article {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            updated_at: row.get(9),
        };
        user_articles.push(article);
    }
    user_articles
}

pub fn get_user_comments(user_id: i32) -> Vec<UserComment> {
    let conn = get_conn();
    let u_id = user_id;
    let mut user_comments: Vec<UserComment> = vec![];
    for row in &conn.query("SELECT
                            comment.aid, comment.uid, comment.content, comment.createtime, 
                            article.id, article,uid, article.category, article.status,
                            article.comments_count, article.title, article.raw, article.cooked, 
                            article.created_at, article.updated_at
                           FROM comment, article WHERE comment.aid = article.id AND comment.uid = $1 ORDER BY comment.id ",&[&u_id]).unwrap() {
        let comment = UserComment {
            id: row.get(5),
            uid: row.get(6),
            category: row.get(7),
            status: row.get(8),
            comments_count: row.get(9),
            title: row.get(10),
            raw: row.get(11),
            cooked: row.get(12),
            created_at: row.get(13),
            updated_at: row.get(14),
            comment_id: row.get(0),
            comment_aid: row.get(1),
            comment_uid: row.get(2),
            comment_content: row.get(3),
            comment_createtime: row.get(4),
        };
        user_comments.push(comment);
    }
    
    user_comments
}

pub fn get_user_messages(user_id: i32) -> Vec<UserMessage> {
    let conn = get_conn();
    let u_id = user_id;
    let mut user_messages: Vec<UserMessage> = vec![];
    for row in &conn.query("SELECT m.status, m.created_at, c.raw, c.cooked, u.id as user_id, u.username,
         u.email, a.id AS article_id, a.title AS article_title 
         FROM message AS m 
         JOIN users AS u ON m.from_uid = u.id 
         JOIN article AS a ON a.id=m.aid 
         JOIN comment AS c ON c.id=m.cid 
         WHERE to_uid= $1 ORDER BY created_at DESC;",&[&u_id]).unwrap() {
        let message = UserMessage {
            message_status: row.get(0),
            message_createtime: row.get(1),
            comment_raw: row.get(2),
            comment_cooked: row.get(3),
            from_uid: row.get(3),
            from_uid_name: row.get(4),
            from_uid_email: row.get(5),
            article_id: row.get(6),
            article_title: row.get(7),
        };
        user_messages.push(message);
    }
    user_messages
}

