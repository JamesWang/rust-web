use warp::reject::Reject;
use crate::types::question::{Question, QuestionId};
use std::str::FromStr;
use warp::filters::{cors::CorsForbidden, body::BodyDeserializeError};
use crate::storage::store::Store;
use std::collections::HashMap;
use crate::types::pagination::extract_pagination;
use warp::http::StatusCode;
use handle_errors::Error;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}


pub async fn get_questions(
    params: HashMap<String, String>, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
    //log::info!("Starting querying questions");
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;        
        let res = &res[pagination.start..pagination.end];
    } else {
        ;
        //log::info!("No Pagination used, returning all questions");
    }
    Ok(warp::reply::json(&res))
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