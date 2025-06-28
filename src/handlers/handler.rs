use warp::reject::Reject;
use crate::models::question::{Question, QuestionId};
use std::str::FromStr;
use warp::filters::cors::CorsForbidden;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}


pub async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
   let question = Question::new(
        QuestionId::from_str("1").expect("No Id Provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    
    match question.id().parse::<u32>() {
        Ok(_) => Ok(warp::reply::json(&question)),
        Err(_) => Err(warp::reject::custom(InvalidId)),
    }    
}

pub async fn return_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Error occurred: {:?}", err);
    if let Some(error) = err.find::<CorsForbidden>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ));
    } else
    if let Some(_InvalidId) = err.find::<InvalidId>() {
        return Ok(warp::reply::with_status(
            "Invalid ID provided".to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY
        )); 
    } else {
        // Handle other types of errors
        eprintln!("Unhandled error: {:?}", err);
        Ok(warp::reply::with_status(
            "Route not found or internal error".to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}