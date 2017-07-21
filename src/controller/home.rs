use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use controller::user::UserOr;

#[get("/",rank = 2)]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    // println!("=====index =========",);
    Template::render("index", &context)
}

#[get("/")]
pub fn user_index(user: UserOr) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    context.insert("username", user.0);
    // println!("=====index   user=========",);
    Template::render("index", &context)
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