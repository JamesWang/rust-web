use sqlx::error::Error as SqlxError;
use std::collections::HashMap;
use warp::filters::{body::BodyDeserializeError, cors::CorsForbidden};
use warp::http::StatusCode;
use argon2::{
    Error as ArgonError
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    WrongPassword,
    ArgonLibraryError(ArgonError),
    QuestionAlreadyExists,
    DatabaseQueryError(SqlxError),
    HashingError(String),
    CannotDecryptToken,
    Unauthorized,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ParseError(err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameters"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::WrongPassword => write!(f, "Wrong password"),
            Error::ArgonLibraryError(e) => write!(f, "Cannot verify password"),
            Error::QuestionAlreadyExists => write!(f, "Question already exists"),
            Error::DatabaseQueryError(e) => write!(f, "Database query error: {:?}", e),
            Error::HashingError(e) => write!(f, "Password hashing error: {}", e),
            Error::CannotDecryptToken => write!(f, "Cannot decrypt token"),
            Error::Unauthorized => write!(f, "Unauthorized access"),
        }
    }
}
const DUPLICATE_KEY: u32 = 23505;

pub async fn return_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Error occurred: {:?}", err);
    if let Some(error) = err.find::<Error>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::BAD_REQUEST,
        ));
    } else if let Some(error) = err.find::<CorsForbidden>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ));
    } else if let Some(error) = err.find::<BodyDeserializeError>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    } else if let Some(Error::WrongPassword) = err.find() {
        return Ok(warp::reply::with_status(
            "E-Mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }
    /* else
    if let Some(_InvalidId) = err.find::<InvalidId>() {
        return Ok(warp::reply::with_status(
            "Invalid ID provided".to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY
        ));
    }  */
    else {
        if let Some(Error::DatabaseQueryError(sqlx_error)) = err.find() {
            match sqlx_error {
                sqlx::Error::Database(db_error) => {
                    if db_error.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                        return Ok(warp::reply::with_status(
                            "Duplicate key error".to_string(),
                            StatusCode::CONFLICT,
                        ));
                    } else {
                        return Ok(warp::reply::with_status(
                            format!("Database error: {}", db_error.message()),
                            StatusCode::UNPROCESSABLE_ENTITY,
                        ));
                    }
                }
                _ => {
                    return Ok(warp::reply::with_status(
                        format!("Database error: {}", sqlx_error.to_string()),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ));
                }
            }
        } else {
            // Handle other types of errors
            eprintln!("Unhandled error: {:?}", err);
            Ok(warp::reply::with_status(
                "Route not found or internal error".to_string(),
                warp::http::StatusCode::NOT_FOUND,
            ))
        }
    }
}

impl warp::reject::Reject for Error {}
