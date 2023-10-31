use rocket::http::Status;
use rocket::serde::json::Json;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::{sqlx, Connection, Database};

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::answers::model::Answer;
use crate::app::modules::answers::services::repository as answers_repository;

pub async fn get_show_admin(
    db: Connection<Db>,
    _admin: UserInClaims,
    id: i32,
) -> Result<Json<Answer>, Status> {
    let answer = answers_repository::get_by_id(db, id).await;

    match answer {
        Ok(answer) => Ok(Json(answer)),
        Err(_) => {
            println!("Error: get_show_admin; Answer not obtained.");
            Err(Status::NotFound)
        }
    }
}

pub async fn get_show_multi_admin(
    db: Connection<Db>,
    _admin: UserInClaims,
    ids: Vec<i32>,
) -> Result<Json<Vec<Answer>>, Status> {
    let answers = answers_repository::get_by_ids(db, ids).await;

    match answers {
        Ok(answers) => Ok(Json(answers)),
        Err(_) => {
            println!("Error: get_show_multi_admin; Answers not obtained.");
            Err(Status::NotFound)
        }
    }
}
