use warp::Filter;
use crate::handlers::handler::get_questions;
use crate::handlers::handler::return_error;
use crate::storage;

pub async fn minimal_http_svr() {
    let store = storage::store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    
    let routes = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query()) // Replace YourQueryStruct with your actual struct
        .and(store_filter)
        .and_then(get_questions);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        //The following line add listed headers to the allowed headers
        //.allow_header(vec!["x-foo", "x-bar"])
        //The following line means totally allowed headers
        .allow_headers(vec!["not_in_the_request"]);

    // Start the warp server on port 8083
    warp::serve(routes.with(cors))
        .run(([0, 0, 0, 0], 8083))
        .await;
}