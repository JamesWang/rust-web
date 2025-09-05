use std::future;

use crate::storage::store::Store;
use crate::types::account::{Account, AccountId, NewAccount, Session};
use argon2::Config;
use handle_errors::Error;
use rand::Rng;
use warp::http::StatusCode;
use chrono::prelude::*;
use warp::Filter;


const SALT_FOR_TOKEN: &str = "RANDOM WORDS WINTER MACINTOSH PC";


pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes())?;
    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account created", StatusCode::OK)),
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Failed to create account: {:?}", e);
            Err(warp::reject::custom(Error::DatabaseQueryError(e)))
        }
    }
}

pub fn hash_password(password: &[u8]) -> Result<String, Error> {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    let hashed = argon2::hash_encoded(password, &salt, &config)
        .map_err(|e| Error::HashingError(e.to_string()))?;
    Ok(hashed)
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => {
            match verify_password(&account.password, &login.password.as_bytes()) {            
                Ok(verified) => {
                    if verified {
                        Ok(warp::reply::json(&issue_token(&account.id.expect("id not found"))))
                    } else {
                        Err(warp::reject::custom(Error::WrongPassword))
                    }                
                },
                Err(e) => Err(warp::reject::custom(Error::ArgonLibraryError(e))),
            }
        },
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Failed to fetch account: {:?}", e);
            Err(warp::reject::custom(e))
        }
    }
}

fn verify_password(hashed_password: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hashed_password, password)
}

fn issue_token(account_id: &AccountId) -> String {
    // In a real application, you would generate a JWT or similar token here.
    let crrent_date_time = Utc::now();
    let dt = crrent_date_time + chrono::Duration::days(1);
    let state = serde_json::to_string(account_id).expect("Failed to serialize account ID");
    tracing::event!(tracing::Level::INFO, "Issuing token for account: {}", state);
    return paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(SALT_FOR_TOKEN.as_bytes()))        
        .set_expiration(&dt)
        .set_not_before(&crrent_date_time)
        .set_claim("account_id", serde_json::to_value(account_id).unwrap())
        .build()
        .expect("Failed to build token with builder");
}

pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let parsed_token = paseto::tokens::validate_local_token(
        &token,
        None,
        &Vec::from(SALT_FOR_TOKEN.as_bytes()),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|e| handle_errors::Error::CannotDecryptToken)?;

    serde_json::from_value::<Session>(parsed_token).map_err(|_|{
        handle_errors::Error::CannotDecryptToken
    })
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
            let token = match verify_token(token) {
                Ok(t) => t,
                Err(_) => return future::ready(Err(warp::reject::reject())),
            };
            future::ready(Ok(token))
        })
}