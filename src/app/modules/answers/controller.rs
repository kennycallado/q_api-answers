use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::claims::AccessClaims;
use crate::config::database::Db;

use crate::app::modules::answers::handlers::{create, index, show, update};

use crate::app::modules::answers::model::{Answer, NewAnswer};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        option_index,
        option_show,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        post_multiple,
        post_multiple_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/")]
pub async fn option_index() -> Status {
    Status::Ok
}

#[options("/<_id>")]
pub async fn option_show(_id: i32) -> Status {
    Status::Ok
}


#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<Answer>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!("Error: get_index; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(db: Db, claims: AccessClaims, id: i32) -> Result<Json<Answer>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(db, claims.0.user, id).await,
        _ => {
            println!("Error: get_show; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[get("/<_id>", rank = 102)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/multiple", data="<ids>", rank = 1)]
pub async fn post_multiple(db: Db, claims: AccessClaims, ids: Json<Vec<i32>>) -> Result<Json<Vec<Answer>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_multi_admin(db, claims.0.user, ids.into_inner()).await,
        "robot" => show::get_show_multi_admin(db, claims.0.user, ids.into_inner()).await,
        _ => {
            println!("Error: post_multiple; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[post("/multiple", data="<_ids>", rank = 2)]
pub async fn post_multiple_none(_ids: Json<Vec<i32>>) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_answer>", rank = 1)]
pub async fn post_create(db: Db, claims: AccessClaims, new_answer: Json<NewAnswer>) -> Result<Json<Answer>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(db, claims.0.user, new_answer.into_inner()).await,
        _ => {
            println!("Error: post_create; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[post("/", data = "<_new_answer>", rank = 2)]
pub async fn post_create_none(_new_answer: Json<NewAnswer>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_answer>", rank = 1)]
pub async fn put_update(db: Db, claims: AccessClaims, id: i32, new_answer: Json<NewAnswer>) -> Result<Json<Answer>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(db, claims.0.user, id, new_answer.into_inner()).await,
        _ => {
            println!("Error: put_update; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[put("/<_id>", data = "<_new_answer>", rank = 2)]
pub async fn put_update_none(_id: i32, _new_answer: Json<NewAnswer>) -> Status {
    Status::Unauthorized
}
