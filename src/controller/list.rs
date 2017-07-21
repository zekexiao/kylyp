use rocket_contrib::Template;
use handler::content::index_list;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
}

#[get("/info")]
pub fn info() -> Template {
    let context = TemplateContext {
        title: String::from("Forum"),
    };
    index_list();
    Template::render("list", &context)
}