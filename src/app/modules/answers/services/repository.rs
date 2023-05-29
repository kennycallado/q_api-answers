use diesel::prelude::*;
// use diesel::sql_types::Timestamptz;

use crate::config::database::Db;
use crate::database::schema::answers;

use crate::app::modules::answers::model::{Answer, NewAnswer};

pub async fn get_all(db: &Db) -> Result<Vec<Answer>, diesel::result::Error> {
    let answers = db.run(move |conn| answers::table.load::<Answer>(conn) ).await;

    answers
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Answer, diesel::result::Error> {
    let answer = db.run(move |conn| {
        answers::table.find(id).first::<Answer>(conn)
    }).await;

    answer
}

pub async fn get_by_ids(db: &Db, ids: Vec<i32>) -> Result<Vec<Answer>, diesel::result::Error> {
    let answers = db.run(move |conn| {
        answers::table.filter(answers::id.eq_any(ids)).load::<Answer>(conn)
    }).await;

    answers
}

pub async fn create(db: &Db, new_answer: NewAnswer) -> Result<Answer, diesel::result::Error> {
    let answer = db.run(move |conn| {
        diesel::insert_into(answers::table)
            .values(&new_answer)
            .get_result::<Answer>(conn)
    }).await;

    answer
}

pub async fn update(db: &Db, id: i32, new_answer: NewAnswer) -> Result<Answer, diesel::result::Error> {
    let answer = db.run(move |conn| {
        diesel::update(answers::table.find(id))
            .set(&new_answer)
            .get_result::<Answer>(conn)
    }).await;

    answer
}
