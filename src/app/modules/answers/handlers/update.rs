use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

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
