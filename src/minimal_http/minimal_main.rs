use warp::Filter;
use crate::handlers::handler::get_questions;
use crate::handlers::handler::return_error;

pub async fn minimal_http_svr() {
    let routes = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);


    // Start the warp server on port 8083
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8083))
        .await;
}