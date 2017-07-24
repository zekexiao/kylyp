use rocket_contrib::Template;
use controller::user::UserOr;
use std::collections::HashMap;
use std::fmt::Debug;
use handler::content::Ulist;


#[derive(Serialize)]
struct TemplateContext {
    datas: Vec<Ulist>,
    username: String,
}


#[get("/info")]
pub fn info() -> Template {
    let mut context = HashMap::new();
    context.insert("", "".to_string());
    Template::render("login", &context)
}

