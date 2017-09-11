use rocket_contrib::Template;
use std::collections::HashMap;
use rocket::request::Form;
use controller::user::{UserId,UserOr};
use handler::content::{ get_article_by_aid,get_comment_by_aid,add_comment_by_aid};
use handler::content::{ Ucomment,Uarticle,article_list,add_article_by_uid};

#[derive(Debug,Serialize)]
struct TemplateContext {
    article: Uarticle,
    comments: Vec<Ucomment>,
    username: String,
    user_id: i32,
}

#[derive(Debug,Serialize)]
struct TemplateArticle {
    datas: Vec<Uarticle>,
    username: String,
    user_id: i32,
}
#[derive(FromForm,Debug)]
pub struct DataArticle {
    pub category: String,
    pub title: String,
    pub content: String,
}

#[derive(FromForm,Debug)]
pub struct DataComment {
    pub aid: Option<i32>,
    pub content: String,
}

#[get("/<aid>", rank = 2)]
pub fn article_nouser( aid: i32) -> Template {
    let article_data = get_article_by_aid(aid );
    let comment_data = get_comment_by_aid(aid);
    let context = TemplateContext {
        article: article_data,
        comments: comment_data,
        username: "".to_string(),
        user_id: 0,
    };
    Template::render("article", &context)
}

#[get("/<aid>")]
pub fn article(user: UserOr, aid: i32, user_id: UserId) -> Template {
    let article_data = get_article_by_aid(aid );
    let comment_data = get_comment_by_aid(aid);
    let context = TemplateContext {
        article: article_data,
        comments: comment_data,
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("article", &context)
}

#[get("/addcomment?<data_comment>")]
pub fn add_comment(user: UserOr, user_id: UserId, data_comment: DataComment)  {
    let uid = user_id.0;
    if let Some(aid) = data_comment.aid {
        let use_aid = aid;
        let use_content = data_comment.content;
        add_comment_by_aid(use_aid, uid, &use_content);
    } else {
        "Something Wrong!".to_string();
    }
}

#[get("/new")]
pub fn new(user: UserOr, user_id: UserId) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.0);
    context.insert("user_id", user_id.0.to_string());
    Template::render("new", &context)
}

#[post("/addarticle", data = "<data_article>")]
fn add_article(user: UserOr, user_id: UserId, data_article: Form<DataArticle>)  -> Template {
    let data = data_article.get();
    let uid = user_id.0;
    let category = &data.category;
    let title = &data.title;
    let content = &data.content;
    add_article_by_uid(uid, &category, &title, &content);
    let datas = article_list();
    let context = TemplateArticle {
        datas: datas,
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("index", &context)
}