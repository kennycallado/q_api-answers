use rocket::http::Status;
use rocket::serde::json::Json;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::{sqlx, Database, Connection};

use crate::app::modules::answers::handlers::{create, index, show, update};
use crate::app::modules::answers::model::{Answer, NewAnswer};
use crate::app::providers::guards::claims::AccessClaims;
use crate::database::connection::Db;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        post_show_multiple,
        post_show_multiple_none,
        post_create,
        post_create_none,
        post_create_multiple,
        post_create_multiple_none,
        put_update,
        put_update_none,
        put_update_multiple,
        put_update_multiple_none,
    ]
}

#[options("/<_..>")]
pub async fn options_all() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Connection<Db>, claims: AccessClaims) -> Result<Json<Vec<Answer>>, Status> {
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
pub async fn get_show(db: Connection<Db>, claims: AccessClaims, id: i32) -> Result<Json<Answer>, Status> {
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

#[post("/show/multiple", data="<ids>", rank = 1)]
pub async fn post_show_multiple(db: Connection<Db>, claims: AccessClaims, ids: Json<Vec<i32>>) -> Result<Json<Vec<Answer>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_multi_admin(db, claims.0.user, ids.into_inner()).await,
        "robot" => show::get_show_multi_admin(db, claims.0.user, ids.into_inner()).await,
        _ => {
            println!("Error: post_multiple; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[post("/show/multiple", data="<_ids>", rank = 2)]
pub async fn post_show_multiple_none(_ids: Json<Vec<i32>>) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_answer>", rank = 1)]
pub async fn post_create(db: Connection<Db>, claims: AccessClaims, new_answer: Json<NewAnswer>) -> Result<Json<Answer>, Status> {
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

#[post("/multiple", data="<new_answers>", rank = 1)]
pub async fn post_create_multiple(db: Connection<Db>, claims: AccessClaims, new_answers: Json<Vec<NewAnswer>>) -> Result<Json<Vec<Answer>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_multi_admin(db, claims.0.user, new_answers.into_inner()).await,
        "robot" => create::post_create_multi_admin(db, claims.0.user, new_answers.into_inner()).await,
        _ => {
            println!("Error: post_multiple; Role not handled");
            Err(Status::Unauthorized)
        },
    }

}

#[post("/multiple", data="<_new_answers>", rank = 2)]
pub async fn post_create_multiple_none(_new_answers: Json<Vec<NewAnswer>>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_answer>", rank = 101)]
pub async fn put_update(db: Connection<Db>, claims: AccessClaims, id: i32, new_answer: Json<NewAnswer>) -> Result<Json<Answer>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(db, claims.0.user, id, new_answer.into_inner()).await,
        _ => {
            println!("Error: put_update; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[put("/<_id>", data = "<_new_answer>", rank = 102)]
pub async fn put_update_none(_id: i32, _new_answer: Json<NewAnswer>) -> Status {
    Status::Unauthorized
}

#[put("/multiple", data = "<new_answers>", rank = 1)]
pub async fn put_update_multiple(db: Connection<Db>, claims: AccessClaims, new_answers: Json<Vec<Answer>>) -> Result<Json<Vec<Answer>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_multi_admin(db, claims.0.user, new_answers.into_inner()).await,
        "robot" => update::put_update_multi_admin(db, claims.0.user, new_answers.into_inner()).await,
        _ => {
            println!("Error: put_update_multiple; Role not handled");
            Err(Status::Unauthorized)
        },
    }
}

#[put("/multiple", data = "<_new_answers>", rank = 2)]
pub async fn put_update_multiple_none(_new_answers: Json<Vec<Answer>>) -> Status {
    Status::Unauthorized
}
