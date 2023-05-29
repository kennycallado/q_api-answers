use serde::{Deserialize, Serialize};

use crate::database::schema::answers;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub answer: String
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = answers)]
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
