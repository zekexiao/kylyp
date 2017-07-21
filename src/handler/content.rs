use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use model::db::establish_connection;
use std::fmt::Debug;
use model::user::{User,NewUser};
use model::container::{List,NewList,NewReply};


pub fn index_list() {
    use utils::schema::user::dsl::{user,id};
    use utils::schema::list::dsl::{list,uid};
    use utils::schema::reply::dsl::{reply,pid};
    let connection = establish_connection();
    // let list_result = list.find(uid).get_result::<List>(&connection);   // 根据表uid查某个表单个数据
    let list_result = list.load::<List>(&connection);       // 查某个表全部数据
    println!("========={:?}==n==========",list_result);
    // let user_result =  user.load::<User>(&connection);  // 查某个表全部数据
    // println!("========={:?}============",user_result);
     let users = try!(user::find(1).first(&connection));
    let lists = List::belonging_to(&users).load(&connection);
}