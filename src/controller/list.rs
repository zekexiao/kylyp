use rocket_contrib::Template;
use controller::user::UserOr;
use std::collections::HashMap;
use std::fmt::Debug;
use handler::content::Ulist;
// use handler::content::get_list_by_id;


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


// #[get("/<pid>")]
// pub fn toptic(user: UserOr) {
//     let data = get_list_by_id();
//     let context = TemplateContext {
//         data: data,
//         username: user.0,
//     };
//     Template::render("list", &context)
// }
