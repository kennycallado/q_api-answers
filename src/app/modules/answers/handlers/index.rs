use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::answers::model::Answer;
use crate::app::modules::answers::services::repository as answers_repository;

pub async fn get_index_admin(db: Db, _admin: UserInClaims) -> Result<Json<Vec<Answer>>, Status> {
    let answers = answers_repository::get_all(&db).await;

    match answers {
        Ok(answers) => Ok(Json(answers)),
        Err(_) => {
            println!("Error: get_index_admin; Answers not obtained.");
            Err(Status::InternalServerError)
        },
    }
}
