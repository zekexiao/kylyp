use rocket_contrib::Template;
use std::collections::HashMap;
use controller::user::{UserId,UserOr};
use model::container::{List,Reply,NewList,NewReply};
use handler::content::{ get_list_by_id,get_reply_by_pid,add_reply_pid};

#[derive(Debug,Serialize)]
struct TemplateContext {
    toptic: List,
    replys: Vec<Reply>,
    username: String,
}

#[derive(FromForm,Debug)]
pub struct DataReply {
    pub pid: Option<i32>,
    pub content: String,
}

#[get("/<pid>", rank = 2)]
pub fn toptic_no( pid: i32) -> Template {
    let toptic_data = get_list_by_id(pid );
    let reply_data = get_reply_by_pid(pid);
    let context = TemplateContext {
        toptic: toptic_data,
        replys: reply_data,
        username: "".to_string(),
    };
    Template::render("theme", &context)
}

#[get("/<pid>")]
pub fn toptic(user: UserOr, pid: i32) -> Template {
    let toptic_data = get_list_by_id(pid );
    let reply_data = get_reply_by_pid(pid);
    let context = TemplateContext {
        toptic: toptic_data,
        replys: reply_data,
        username: user.0,
    };
    Template::render("theme", &context)
}

#[get("/addreply?<data_reply>")]
pub fn reply(user: UserOr,  user_id: UserId, data_reply: DataReply)  {
    let uid = user_id.0;
    if let Some(pid) = data_reply.pid {
        let use_pid = pid;
        let use_content = data_reply.content;
        add_reply_pid(use_pid, uid, &use_content);
    } else {
        "Something Wrong!".to_string();
    }
}

#[get("/new")]
pub fn new(user: UserOr) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.0);
    Template::render("new", &context)
}