use rocket_contrib::Template;
use controller::user::UserOr;
use std::collections::HashMap;
use model::container::{List,NewList,NewReply};
use std::fmt::Debug;
use handler::content::{Ulist,date_index, get_list_by_id};


#[derive(Debug,Serialize)]
struct TemplateContext {
    data: List,
}


#[get("/info")]
pub fn info() -> Template {
    let mut context = HashMap::new();
    context.insert("", "".to_string());
    Template::render("login", &context)
}


#[get("/<pid>")]
pub fn toptic(user: UserOr, pid: i32) -> Template {
    let data = get_list_by_id(pid );
    let context = TemplateContext {
        data: data,
    };
    Template::render("list", &context)
}
