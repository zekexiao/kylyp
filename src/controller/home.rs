use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use controller::user::UserOr;
use handler::content::date_list;
use handler::content::Ulist;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    datas: Vec<Ulist>,
    username: String,
}

#[get("/",rank = 2)]
pub fn index() -> Template {
    let datas = date_list();
    let context = TemplateContext {
        title: "Forum".to_string(),
        datas: datas,
        username: "".to_string(),
    };
    Template::render("index", &context)
}


#[get("/")]
pub fn index_user(user: UserOr) -> Template {
    let datas = date_list();
    let context = TemplateContext {
        title: "Forum".to_string(),
        datas: datas,
        username: user.0,
    };
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