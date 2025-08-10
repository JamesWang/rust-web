use warp::reject::Reject;
use crate::types::question::{Question, QuestionId, NewQuestion};
use crate::types::answer::{Answer, AnswerId, NewAnswer};
use std::str::FromStr;
use warp::filters::{cors::CorsForbidden, body::BodyDeserializeError};
use crate::storage::store::Store;
use std::collections::HashMap;
use crate::types::pagination::{extract_pagination, Pagination};
use warp::http::StatusCode;
use handle_errors::Error;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}
use tracing::{event, instrument, Level};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "Fetching questions with params: {:?}", params);
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }
    //info!(pagination = false);
    let res: Vec<Question> = match store.get_questions(pagination.limit, pagination.offset)
    .await {
        Ok(questions) => questions,
        Err(e) => {
            //event!(Level::ERROR, "Database query error: {}", e);
            tracing::event!(Level::ERROR, "Database query error: {:?}", e);
            return Err(warp::reject::custom(e));
        }
    };
    event!(Level::INFO, "Fetched {} questions", res.len());
    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    new_question: NewQuestion, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        tracing::event!(Level::ERROR, "Failed to add question: {:?}", e);
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: i32, 
    question: Question, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(QuestionId::new(id), question).await {
        Ok(res) => res,
        Err(e) => {
            tracing::event!(Level::ERROR, "Failed to update question: {:?}", e);
            return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
        }
    };    
    Ok(warp::reply::json(&res))
}

pub async fn delete_question(
    id: i32, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(QuestionId::new(id)).await {
        tracing::event!(Level::ERROR, "Failed to delete question: {:?}", e);
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }
    // Assuming delete_question returns a Result indicating success or failure
    // If it returns a specific type, you can adjust the response accordingly
    Ok(warp::reply::with_status(format!("Question {} deleted", id), StatusCode::OK))
}


pub async fn add_answer(store: Store, new_answer: NewAnswer) -> Result<impl warp::Reply, warp::Rejection> {
    // Implementation for adding an answer
    // This function will need to be defined in the `types::answer` module
    match store.add_answer(new_answer).await {
        Ok(_answer) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => {
            tracing::event!(Level::ERROR, "Failed to add answer: {:?}", e);
            return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
        }
    }
}