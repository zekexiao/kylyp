#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(custom_attribute)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen; 
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate dotenv;
extern crate chrono;

#[macro_use] mod controller;
#[macro_use] mod handler;
mod model;
mod utils;

use rocket_contrib::Template;
use controller::{home,user,list};


fn main() {
    rocket::ignite()
        .mount("/", routes![home::public,home::index_user,home::index,home::add_toptic])
        .mount("/user",routes![user::register,user::login_register,user::register_post,
                               user::login_user,user::login,user::login_post,user::user_page,user::user_page_login,user::logout])
        .mount("/list",routes![list::info])
        .attach(Template::fairing())
        .catch(errors![home::not_found])
        .launch();
    
}

