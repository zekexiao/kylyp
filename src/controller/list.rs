use rocket_contrib::Template;
use controller::user::{UserId,UserOr};
use std::collections::HashMap;
use model::container::{List,Reply,NewList,NewReply};
use rocket::request::{self,Form, FlashMessage,FromRequest,Request};
use std::fmt::Debug;
use handler::content::{Ulist,date_index, get_list_by_id,get_reply_by_pid,add_reply};

#[derive(Debug,Serialize)]
struct TemplateContext {
    toptic: List,
    replys: Vec<Reply>,
    username: String,
}

#[derive(FromForm,Debug)]
pub struct DataReply {
    pub content: String,
    pub createtime: String,
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
    Template::render("list", &context)
}

#[post("/<pid>/addreply", data = "<data_reply>")]
pub fn reply(user: UserOr, pid: i32, user_id: UserId, data_reply: Form<DataReply>) -> Template {
    println!("=====================" );
    {
    let data = data_reply.get();
    let uid = user_id.0;
    let content = &data.content;
    println!("========={:?},{:?},{:?}============" ,uid,pid,content);
    add_reply(uid, pid, &content);
    }
    let toptic_data = get_list_by_id(pid );
    let reply_data = get_reply_by_pid(pid);
    let context = TemplateContext {
        toptic: toptic_data,
        replys: reply_data,
        username: user.0,
    };
    Template::render("list", &context)

}