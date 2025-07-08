use warp::Filter;
use crate::routes::handler::get_questions;
use crate::storage;
use handle_errors::return_error;

pub async fn minimal_http_svr() {
    let store = storage::store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    
    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query()) // Replace YourQueryStruct with your actual struct
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::body::json())
        //order matters, so the store_filter must be after the body::json()
        //because the body::json() will try to deserialize the body into a Question struct
        //and the store_filter will be used to access the store
        //add_question(Question, Store) expects Question first and Store second
        .and(store_filter.clone())
        .and_then(crate::routes::handler::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>()) // Assuming the ID is a String
        .and(warp::path::end())
        .and(warp::body::json()) // get Question struct from the body
        .and(store_filter.clone()) // get the store for update_question
        .and_then(crate::routes::handler::update_question); // You might want to change this to an actual update handler

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        //The following line add listed headers to the allowed headers
        //.allow_header(vec!["x-foo", "x-bar"])
        //The following line means totally allowed headers
        .allow_headers(vec!["not_in_the_request"]);

    let routes = get_question
        .or(add_question)
        .or(update_question)
        .with(cors)
        .recover(return_error);
    // Start the warp server on port 8083
    //println!("Starting server on http://localhost:8083");
    warp::serve(routes).run(([0, 0, 0, 0], 8083)).await;
}