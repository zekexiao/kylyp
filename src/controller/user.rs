use diesel;
use diesel::prelude::*;
use rocket_contrib::Template;
use rocket::request::{self,Form, FlashMessage,FromRequest,Request};
use rocket::response::{Redirect,Flash};
use model::db::establish_connection;
use model::pg::get_conn;
use model::user::{User,NewUser};
use model::article::{Article,Comment};
use rocket::http::{Cookie, Cookies};
use rocket::http::RawStr;
use std::collections::HashMap;
use rocket::outcome::IntoOutcome;
use chrono::prelude::*;
use handler::content::{UserComment,UserMessage,get_user_info,get_user_articles,get_user_comments,get_user_messages};
use chrono::{DateTime,Utc};

#[derive(Debug,Serialize)]
pub struct Uid {
    id: i32,
}

#[derive(Debug)]
pub struct UserOr(pub String);
#[derive(Debug)]
pub struct UserId(pub i32);

impl<'a, 'r> FromRequest<'a, 'r> for UserOr {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserOr, ()> {
        request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserOr(id))
            .or_forward(())
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserId, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserId(id))
            .or_forward(())
    }
}

#[derive(FromForm)]
struct UserRegister {
    email: String,
    username: String,
    password: String,
    password2: String,
}

#[derive(FromForm,Debug)]
struct UserLogin {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct UserInfo {
    login_user: Option<User>,
    user_articles: Vec<Article>,
    user_comments: Vec<UserComment>,
    user_messages: Vec<UserMessage>,
    username: String,
    user_id: i32,
}

#[get("/<name>")]
pub fn user_page_login(name: &RawStr,user: UserOr,user_id: UserId,flash: Option<FlashMessage>) -> Template {
    if name == &user_id.0.to_string() {
        let this_user = get_user_info(&user_id);
        let articles = get_user_articles(&user_id);
        let comments = get_user_comments(&user_id);
        let messages = get_user_messages(&user_id);
        let context = UserInfo {
            login_user: this_user,
            user_articles: articles,
            user_comments: comments,
            user_messages: messages,
            username: user.0,
            user_id: user_id.0,
        };
        Template::render("user", &context)
    }else{
        let mut context = HashMap::new();
        if let Some(ref msg) = flash {
            context.insert("flash","该用户不存在".to_string());
        }
        Template::render("login", &context)
    }
}

#[get("/register", rank = 2)]
pub fn register(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg().to_string());
    }
    Template::render("register", &context)
}

#[get("/register")]
pub fn login_register(user: UserOr) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.0);
    Template::render("index", &context)
}

#[get("/login", rank = 2)]
pub fn login(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg().to_string());
    }
    Template::render("login", &context)
}

#[get("/login")]
pub fn login_user(user: UserOr) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.0);
    Template::render("index", &context)
}

#[post("/register",data = "<user_form>")]
fn register_post(user_form: Form< UserRegister>) -> Result<Redirect, String> {
    let post_user = user_form.get();
    use utils::schema::users;
    if &post_user.password == &post_user.password2 {
            let connection = establish_connection();
            let new_user = NewUser {
                email: &post_user.email,
                username: &post_user.username,
                password: &post_user.password,
                created_at: Utc::now(), 
            };
            diesel::insert(&new_user).into(users::table).execute(&connection).expect("User is  Exist!");
            Ok(Redirect::to("/user/login"))
    }else {
        Err("password != password2".to_string())
    }
}
// -------------- 方法一 -------------
#[post("/login", data = "<user_form>")]
fn login_post(mut cookies: Cookies, user_form: Form<UserLogin>) -> Flash<Redirect> {
    let post_user = user_form.get();
    let conn = get_conn();
    let mut uid = Uid {id : 0};
    for row in &conn.query("SELECT id FROM users WHERE username =$1  AND password = $2", &[&post_user.username,&post_user.password]).unwrap() {
        uid = Uid {
            id : row.get(0),
        };
    }
    if uid.id != 0 {
            cookies.add_private(Cookie::new("user_id",uid.id.to_string() ));
            cookies.add_private(Cookie::new("username",post_user.username.to_string() ));
            Flash::success(Redirect::to("/"), "Successfully logged in")
            
    }else {
            Flash::error(Redirect::to("/user/login"), "Incorrect")
    } 
}

// -------------- 方法二 -------------
// #[post("/login", data = "<user_form>")]
// fn login_post(mut cookies: Cookies, user_form: Form<UserLogin>) -> Flash<Redirect> {
//     use utils::schema::users::dsl::*;
//     let post_user = user_form.get();
//     let connection = establish_connection();
//     let user_result =  users.filter(&username.eq(&post_user.username)).load::<User>(&connection);
//     let login_user = match user_result {
//         Ok(user_s) => match user_s.first() {
//             Some(a_user) => Some(a_user.clone()),
//             None => None,
//         },
//         Err(_) => None,
//     };
//     match login_user {
//         Some(login_user) => {
//             if &post_user.password == &login_user.password {
//                 cookies.add_private(Cookie::new("username",post_user.username.to_string() ));
//                 cookies.add_private(Cookie::new("user_id",login_user.id.to_string() ));
//                 Flash::success(Redirect::to("/"), "Successfully logged in")
//             }else {
//                 Flash::error(Redirect::to("/user/login"), "Incorrect Password")
//             }
//         },
//         None => Flash::error(Redirect::to("/user/login"), "Incorrect Username"),
//     }
// }

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("username"));
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/user/login"), "Successfully logged out.")
}

