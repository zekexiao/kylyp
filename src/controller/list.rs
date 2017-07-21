use rocket_contrib::Template;
use controller::user::UserOr;
use std::collections::HashMap;
use std::fmt::Debug;
use handler::content::Ulist;
use handler::content::date_list;


#[derive(Serialize)]
struct TemplateContext {
    title: String,
    datas: Vec<Ulist>,
    username: String,
}


#[get("/info", rank = 2)]
pub fn info() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Forum".to_string());
    Template::render("login", &context)
}