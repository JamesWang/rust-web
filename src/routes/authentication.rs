use warp::http::StatusCode;
use crate::storage::store::Store;
use crate::types::account::{Account, NewAccount};
use handle_errors::Error;

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account created", StatusCode::OK)),
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Failed to create account: {:?}", e);
            Err(warp::reject::custom(e))
        }
    }
}