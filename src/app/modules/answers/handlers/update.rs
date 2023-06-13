use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::answers::model::{Answer, NewAnswer};
use crate::app::modules::answers::services::repository as answers_repository;

pub async fn put_update_admin(db: Db, _admin: UserInClaims, id: i32, new_answer: NewAnswer) -> Result<Json<Answer>, Status> {
    let answer = answers_repository::update(&db, id, new_answer).await;

    match answer {
        Ok(answer) => Ok(Json(answer)),
        Err(_) => {
            println!("Error: put_update_admin; Answer not updated.");
            Err(Status::InternalServerError)
        },
    }
}

pub async fn put_update_multi_admin(db: &Db, _admin: UserInClaims, new_answers: Vec<Answer>) -> Result<Json<Vec<Answer>>, Status> {
    let answers = answers_repository::update_multi(db, new_answers).await;

    match answers {
        Ok(answers) => Ok(Json(answers)),
        Err(_) => {
            println!("Error: put_update_multi_admin; Answers not updated.");
            Err(Status::InternalServerError)
        },
    }
}
