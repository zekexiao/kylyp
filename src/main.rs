#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![recursion_limit="128"]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen; 
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate dotenv;
extern crate chrono;
extern crate regex;

#[macro_use] mod controller;
#[macro_use] mod handler;
#[macro_use] mod model;
mod utils;

use rocket_contrib::Template;
use controller::{home,user,article};
use handler::config::configenv;
use rocket::fairing::AdHoc;
use rocket::config::{self, Config};

struct LocalConfig(Config);

#[derive(Debug,Serialize)]
pub struct Envconf {
    address: String,
    port: u16,
}


fn main() {
    rocket::ignite()
        .attach(AdHoc::on_attach(|rocket| {
            let config = rocket.config().clone();
            let mut envconf = Envconf { 
                address: config.address.clone(), 
                port: config.port, 
            };
            configenv(envconf);
            Ok(rocket.manage(LocalConfig(config)))
        }))
        .mount("/", routes![home::public,home::index_user,home::index])
        .mount("/user",routes![user::register,user::login_register,user::register_post,
                               user::login_user,user::login,user::login_post,user::user_page,user::user_page_login,user::logout])
        .mount("/article",routes![article::article,article::add_comment,article::article_nouser,article::new,article::add_article])
        .attach(Template::fairing())
        .catch(errors![home::not_found])
        .launch();
}



