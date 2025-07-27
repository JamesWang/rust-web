use warp::reject::Reject;
use crate::types::question::{Question, QuestionId};
use crate::types::answer::{Answer, AnswerId};
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

pub async fn delete_question(
    id: String, 
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId::new(id)) {
        Some(_) =>{
            return Ok(warp::reply::with_status("Question deleted", StatusCode::OK));
        },
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}


pub async fn add_answer(store: Store, params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Implementation for adding an answer
    // This function will need to be defined in the `types::answer` module
    let answer = Answer {
        id: AnswerId::new("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId::new(params.get("question_id").unwrap().to_string()),
    };
    store.answers.write().await.insert(answer.id.clone(), answer);
    Ok(warp::reply::with_status("Answer added", warp::http::StatusCode::OK))
}