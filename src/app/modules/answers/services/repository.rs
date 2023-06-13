use diesel::prelude::*;

use crate::database::connection::Db;
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

pub async fn create_multi(db: &Db, new_answers: Vec<NewAnswer>) -> Result<Vec<Answer>, diesel::result::Error> {
    let answers = db.run(move |conn| {
        diesel::insert_into(answers::table)
            .values(&new_answers)
            .get_results::<Answer>(conn)
    }).await;

    answers
}

pub async fn update(db: &Db, id: i32, new_answer: NewAnswer) -> Result<Answer, diesel::result::Error> {
    let answer = db.run(move |conn| {
        diesel::update(answers::table.find(id))
            .set(&new_answer)
            .get_result::<Answer>(conn)
    }).await;

    answer
}

pub async fn update_multi(db: &Db, new_answers: Vec<Answer>) -> Result<Vec<Answer>, diesel::result::Error> {
    let ids = new_answers.iter().map(|answer| answer.id).collect::<Vec<i32>>();

    let answers = db.run(move |conn| {
        for answer in new_answers.into_iter() {
            let id = answer.id;
            let new_answer = NewAnswer::from(answer);

            diesel::update(answers::table.find(id))
                .set(&new_answer)
                .execute(conn)
                .expect("Error updating answer");
        }

        answers::table.filter(answers::id.eq_any(ids)).load::<Answer>(conn)
    }).await;

    answers
}
