use rocket_contrib::Template;
use std::collections::HashMap;
use rocket::request::Form;
use controller::user::{UserId,UserOr};
use handler::content::{ get_article_by_aid,get_comment_by_aid,add_comment_by_aid};
use handler::content::{ Ucomment,Uarticle,article_list,add_article_by_uid};
use model::db::ConnDsl;
use model::pg::ConnPg;
use model;

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
    pub raw: String,
}

#[derive(FromForm,Debug)]
pub struct DataComment {
    pub aid: Option<i32>,
    pub raw: String,
}

#[get("/<aid>", rank = 2)]
pub fn article_nouser(conn_pg: ConnPg, conn_dsl: ConnDsl, aid: i32) -> Template {
    let article_data = get_article_by_aid(&conn_pg, aid );
    let comment_data = get_comment_by_aid(&conn_pg, aid);
    let context = TemplateContext {
        article: article_data,
        comments: comment_data,
        username: "".to_string(),
        user_id: 0,
    };
    Template::render("article", &context)
}

#[get("/<aid>")]
pub fn article(conn_pg: ConnPg, conn_dsl: ConnDsl, user: UserOr, aid: i32, user_id: UserId) -> Template {
    let article_data = get_article_by_aid(&conn_pg, aid);
    let comment_data = get_comment_by_aid(&conn_pg, aid);
    let context = TemplateContext {
        article: article_data,
        comments: comment_data,
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("article", &context)
}

#[get("/addcomment?<data_comment>")]
pub fn add_comment(conn_pg: ConnPg, conn_dsl: ConnDsl, user: UserOr, user_id: UserId, data_comment: DataComment)  {
    let uid = user_id.0;
    if let Some(aid) = data_comment.aid {
        let use_aid = aid;
        let use_content = data_comment.raw;
        add_comment_by_aid(&conn_pg, &conn_dsl, use_aid, uid, &use_content);
    } else {
        "Something Wrong!".to_string();
    }
}

#[get("/new")]
pub fn new(conn_dsl: ConnDsl, user: UserOr, user_id: UserId) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.0);
    context.insert("user_id", user_id.0.to_string());
    Template::render("new", &context)
}

#[post("/addarticle", data = "<data_article>")]
fn add_article(conn_pg: ConnPg, conn_dsl: ConnDsl, user: UserOr, user_id: UserId, data_article: Form<DataArticle>)  -> Template {
    let data = data_article.get();
    let uid = user_id.0;
    let category = &data.category;
    let title = &data.title;
    let raw = &data.raw;
    add_article_by_uid(&conn_dsl, uid, &category, &title, &raw);
    let datas = article_list(&conn_pg);
    let context = TemplateArticle {
        datas: datas,
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("index", &context)
}
