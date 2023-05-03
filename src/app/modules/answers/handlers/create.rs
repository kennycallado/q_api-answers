use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::answers::model::{Answer, NewAnswer};
use crate::app::modules::answers::services::repository as answers_repository;

pub async fn post_create_admin(db: Db, _admin: UserInClaims, new_answer: NewAnswer) -> Result<Json<Answer>, Status> {
    let answer = answers_repository::create(&db, new_answer).await;

    match answer {
        Ok(answer) => Ok(Json(answer)),
        Err(_) => {
            println!("Error: post_create_admin; Answer not created.");
            Err(Status::InternalServerError)
        },
    }
}
