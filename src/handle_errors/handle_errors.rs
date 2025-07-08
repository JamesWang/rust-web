use warp::http::StatusCode;
use warp::filters::{cors::CorsForbidden, body::BodyDeserializeError};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    QuestionAlreadyExists,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError( ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameters"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::QuestionAlreadyExists => write!(f, "Question already exists"),
        }
    }
}

pub async fn return_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Error occurred: {:?}", err);
    if let Some(error) = err.find::<Error>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::BAD_REQUEST,
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

impl warp::reject::Reject for Error {}