use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::request::Request;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use controller::user::{UserId,UserOr};
use handler::content::{Uarticle,article_list};

#[derive(Serialize)]
struct TemplateContext {
    datas: Vec<Uarticle>,
    username: String,
    user_id: i32,
}
#[derive(Serialize)]
struct TemplateDoc {
    username: String,
    user_id: i32,
}

#[get("/",rank = 2)]
pub fn index() -> Template {
    let datas = article_list();
    let context = TemplateContext {
        datas: datas,
        username: "".to_string(),
        user_id: 0,
    };
    Template::render("index", &context)
}

#[get("/")]
pub fn index_user(user: UserOr, user_id: UserId) -> Template {
    let datas = article_list();
    let context = TemplateContext {
        datas: datas,
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("index", &context)
}

#[get("/doc",rank = 2)]
pub fn doc() -> Template {
    let mut context = HashMap::new();
    context.insert("No login user", "".to_string());
    Template::render("doc", &context)
}

#[get("/doc")]
pub fn doc_user(user: UserOr, user_id: UserId) -> Template {
    let context = TemplateDoc {
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("doc", &context)
}

#[get("/area",rank = 2)]
pub fn area() -> Template {
    let mut context = HashMap::new();
    context.insert("No login user", "".to_string());
    Template::render("area", &context)
}

#[get("/area")]
pub fn area_user(user: UserOr, user_id: UserId) -> Template {
    let context = TemplateDoc {
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("area", &context)
}

#[get("/news",rank = 2)]
pub fn news() -> Template {
    let mut context = HashMap::new();
    context.insert("No login user", "".to_string());
    Template::render("news", &context)
}

#[get("/news")]
pub fn news_user(user: UserOr, user_id: UserId) -> Template {
    let context = TemplateDoc {
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("news", &context)
}










#[get("/<file..>",rank = 9)]
pub fn public(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}