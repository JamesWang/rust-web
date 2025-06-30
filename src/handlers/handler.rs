use warp::reject::Reject;
use crate::models::question::{Question, QuestionId};
use std::str::FromStr;
use warp::filters::{cors::CorsForbidden, body::BodyDeserializeError};
use crate::storage::store::Store;
use std::collections::HashMap;
use crate::models::page::extract_pagination;
use crate::models::error::Error;
use warp::http::StatusCode;


#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}


pub async fn get_questions(
    params: HashMap<String, String>, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;        
        let res = &res[pagination.start..pagination.end];
    } 
    Ok(warp::reply::json(&res))
}

pub async fn return_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Error occurred: {:?}", err);
    if let Some(error) = err.find::<Error>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ));
    } else
    if let Some(error) = err.find::<CorsForbidden>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ));
    } else if let Some(error) = err.find::<BodyDeserializeError>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }
    /* else
    if let Some(_InvalidId) = err.find::<InvalidId>() {
        return Ok(warp::reply::with_status(
            "Invalid ID provided".to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY
        )); 
    }  */else {
        // Handle other types of errors
        eprintln!("Unhandled error: {:?}", err);
        Ok(warp::reply::with_status(
            "Route not found or internal error".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}
pub async fn add_question(
    question: Question, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut questions = store.questions.write().await;
    if questions.contains_key(&question.id()) {
        return Err(warp::reject::custom(Error::QuestionAlreadyExists));
    }

    questions.insert(question.id().clone(), question);
    Ok(warp::reply::with_status("Question added", warp::http::StatusCode::CREATED))
}

pub async fn update_question(
    id: String, 
    question: Question, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId::new(id)) {
        Some(existing_question) => *existing_question = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    
    Ok(warp::reply::with_status("Question updated", warp::http::StatusCode::OK))
}