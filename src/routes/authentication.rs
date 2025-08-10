use crate::storage::store::Store;
use crate::types::account::{Account, NewAccount};
use argon2::{self, Config};
use handle_errors::Error;
use rand::Rng;
use warp::http::StatusCode;

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
            Err(warp::reject::custom(e))
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
