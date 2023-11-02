#[cfg(feature = "db_diesel")]
use diesel::prelude::*;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::{QueryBuilder, Row};
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::{sqlx, Connection};

#[cfg(feature = "db_diesel")]
use crate::database::schema::answers;

use crate::app::modules::answers::model::{Answer, NewAnswer};
use crate::database::connection::Db;

pub async fn get_all(mut db: Connection<Db>) -> Result<Vec<Answer>, sqlx::Error> {
    let answers = sqlx::query_as!(Answer, "SELECT * FROM answers")
        .fetch_all(&mut **db)
        .await?;

    Ok(answers)
}

pub async fn get_by_id(mut db: Connection<Db>, id: i32) -> Result<Answer, sqlx::Error> {
    let answer = sqlx::query_as!(Answer, "SELECT * FROM answers WHERE id = $1", id)
        .fetch_one(&mut **db)
        .await?;

    Ok(answer)
}

pub async fn get_by_ids(
    mut db: Connection<Db>,
    ids: Vec<i32>,
) -> Result<Vec<Answer>, sqlx::Error> {
    let answers = sqlx::query_as!(Answer, "SELECT * FROM answers WHERE id = ANY($1)", &ids)
        .fetch_all(&mut **db)
        .await?;

    Ok(answers)
}

pub async fn create(
    mut db: Connection<Db>,
    new_answer: NewAnswer,
) -> Result<Answer, sqlx::Error> {
    let answer = sqlx::query_as!(
        Answer,
        "INSERT INTO answers (question_id, answer) VALUES ($1, $2) RETURNING *",
        new_answer.question_id,
        new_answer.answer
    )
    .fetch_one(&mut **db)
    .await?;

    Ok(answer)
}

pub async fn create_multi(
    mut db: Connection<Db>,
    new_answers: Vec<NewAnswer>,
) -> Result<Vec<Answer>, sqlx::Error> {
    let mut query_builder = QueryBuilder::new("INSERT INTO answers (question_id, answer) ");
    let query = query_builder
        .push_values(new_answers, |mut separator, new_answer| {
            separator
                .push_bind(new_answer.question_id)
                .push_bind(new_answer.answer);
        })
        .push(" RETURNING *")
        .build();

    let answers = query
        .fetch_all(&mut **db)
        .await?
        .into_iter()
        .map(|answer| Answer {
            id: answer.get("id"),
            question_id: answer.get("question_id"),
            answer: answer.get("answer"),
        })
        .collect::<Vec<Answer>>();

    Ok(answers)
}

pub async fn update(
    mut db: Connection<Db>,
    id: i32,
    new_answer: NewAnswer,
) -> Result<Answer, sqlx::Error> {
    let answer = sqlx::query_as!(
        Answer,
        "UPDATE answers SET question_id = $1, answer = $2 WHERE id = $3 RETURNING *",
        new_answer.question_id,
        new_answer.answer,
        id
    )
    .fetch_one(&mut **db)
    .await?;

    Ok(answer)
}

pub async fn update_multi(
    mut db: Connection<Db>,
    new_answers: Vec<Answer>,
) -> Result<Vec<Answer>, sqlx::Error> {
    let mut query_builder = QueryBuilder::new(
        "UPDATE answers AS t SET question_id = a.question_id, answer = a.answer FROM ( ",
    );
    let query = query_builder
        .push_values(new_answers, |mut separator, new_answer| {
            separator
                .push_bind(new_answer.id)
                .push_bind(new_answer.question_id)
                .push_bind(new_answer.answer);
        })
        .push(") AS a(id, question_id, answer) WHERE a.id = t.id RETURNING t.*")
        .build();

    let answers = query
        .fetch_all(&mut **db)
        .await?
        .into_iter()
        .map(|answer| Answer {
            id: answer.get("id"),
            question_id: answer.get("question_id"),
            answer: answer.get("answer"),
        })
        .collect::<Vec<Answer>>();

    Ok(answers)
}
