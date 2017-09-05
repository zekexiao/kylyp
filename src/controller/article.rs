use rocket_contrib::Template;
use std::collections::HashMap;
use rocket::request::Form;
use controller::user::{UserId,UserOr};
use model::article::{Article,Comment,NewArticle,NewComment};
use handler::content::{ get_article_by_id,get_comment_by_pid,add_comment_by_pid};
use handler::content::{Uarticle,article_list,add_article_by_uid};

#[derive(Debug,Serialize)]
struct TemplateContext {
    toptic: Article,
    comments: Vec<Comment>,
    username: String,
}

#[derive(Debug,Serialize)]
struct TemplateArticle {
    datas: Vec<Uarticle>,
    username: String,
}
#[derive(FromForm,Debug)]
pub struct DataArticle {
    pub title: String,
    pub content: String,
}

#[derive(FromForm,Debug)]
pub struct DataComment {
    pub pid: Option<i32>,
    pub content: String,
}

#[get("/<pid>", rank = 2)]
pub fn article_nouser( pid: i32) -> Template {
    let toptic_data = get_article_by_id(pid );
    let comment_data = get_comment_by_pid(pid);
    let context = TemplateContext {
        toptic: toptic_data,
        comments: comment_data,
        username: "".to_string(),
    };
    Template::render("article", &context)
}

#[get("/<pid>")]
pub fn article(user: UserOr, pid: i32) -> Template {
    let toptic_data = get_article_by_id(pid );
    let comment_data = get_comment_by_pid(pid);
    let context = TemplateContext {
        toptic: toptic_data,
        comments: comment_data,
        username: user.0,
    };
    Template::render("article", &context)
}

#[get("/addcomment?<data_comment>")]
pub fn comment(user: UserOr,  user_id: UserId, data_comment: DataComment)  {
    let uid = user_id.0;
    if let Some(pid) = data_comment.pid {
        let use_pid = pid;
        let use_content = data_comment.content;
        add_comment_by_pid(use_pid, uid, &use_content);
    } else {
        "Something Wrong!".to_string();
    }
}

#[get("/new")]
pub fn new(user: UserOr) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.0);
    Template::render("new", &context)
}

#[post("/addarticle", data = "<data_article>")]
fn add_article(user: UserOr, user_id: UserId, data_article: Form<DataArticle>)  -> Template {
    let data = data_article.get();
    let uid = user_id.0;
    let title = &data.title;
    let content = &data.content;
    add_article_by_uid(uid, &title,&content);
    let datas = article_list();
    let context = TemplateArticle {
        datas: datas,
        username: user.0,
    };
    Template::render("index", &context)
}