use warp::Filter;
use crate::handlers::handler::get_questions;
use crate::handlers::handler::return_error;

pub async fn minimal_http_svr() {
    let routes = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        //.allow_header(vec!["x-foo", "x-bar"]) ---------this line add listed headers to the allowed headers
        //the following line means totally allowed headers
        .allow_headers(vec!["not_in_the_request"]);

    // Start the warp server on port 8083
    warp::serve(routes.with(cors))
        .run(([0, 0, 0, 0], 8083))
        .await;
}