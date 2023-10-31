use rocket::http::Status;
use rocket::serde::json::Json;


#[cfg(feature = "db_sqlx")]
use rocket_db_pools::{sqlx, Database, Connection};

use crate::app::providers::services::claims::UserInClaims;
use crate::app::modules::answers::model::{Answer, NewAnswer};
use crate::app::modules::answers::services::repository as answers_repository;
use crate::database::connection::Db;

pub async fn post_create_admin(db: Connection<Db>, _admin: UserInClaims, new_answer: NewAnswer) -> Result<Json<Answer>, Status> {
    let answer = answers_repository::create(db, new_answer).await;

    match answer {
        Ok(answer) => Ok(Json(answer)),
        Err(_) => {
            println!("Error: post_create_admin; Answer not created.");
            Err(Status::InternalServerError)
        },
    }
}

pub async fn post_create_multi_admin(db: Connection<Db>, _admin: UserInClaims, new_answers: Vec<NewAnswer>) -> Result<Json<Vec<Answer>>, Status> {
    let answers = answers_repository::create_multi(db, new_answers).await;

    match answers {
        Ok(answers) => Ok(Json(answers)),
        Err(_) => {
            println!("Error: post_create_multi_admin; Answers not created.");
            Err(Status::InternalServerError)
        },
    }
}
