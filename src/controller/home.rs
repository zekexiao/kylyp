use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::request::{self,Form, FlashMessage,FromRequest,Request};
use rocket::http::RawStr;
use rocket::http::Cookies;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use controller::user::UserOr;
use handler::content::{Ulist,date_index,get_add_topic};
use chrono::prelude::*;


#[derive(Serialize)]
struct TemplateContext {
    datas: Vec<Ulist>,
    username: String,
}

#[derive(FromForm,Debug)]
pub struct DataList {
    pub title: String,
    pub content: String,
}

#[get("/",rank = 2)]
pub fn index() -> Template {
    let datas = date_index();
    let context = TemplateContext {
        datas: datas,
        username: "".to_string(),
    };
    Template::render("index", &context)
}


#[get("/")]
pub fn index_user(user: UserOr) -> Template {
    let datas = date_index();
    let context = TemplateContext {
        datas: datas,
        username: user.0,
    };
    Template::render("index", &context)
}

#[post("/addtoptic", data = "<data_list>")]
fn add_topic<'a>(user: UserOr, data_list: Form<DataList>)  -> Template {
    {
    let data = data_list.get();
    let uid = &user.0;
    let title = &data.title;
    let content = &data.content;
    get_add_topic(&uid, &title,&content);
    }
    let datas = date_index();
    let context = TemplateContext {
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