use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::answers;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub answer: String
}

#[cfg_attr(feature = "db_diesel", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = answers))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewAnswer {
    pub question_id: i32,
    pub answer: String,
}

impl From<Answer> for NewAnswer {
    fn from(answer: Answer) -> Self {
        NewAnswer {
            question_id: answer.question_id,
            answer: answer.answer,
        }
    }
}
