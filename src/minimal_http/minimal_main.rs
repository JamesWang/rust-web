use serde::de;
use warp::Filter;
use crate::routes::handler::{add_answer, get_questions};
use crate::storage;
use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;

pub async fn minimal_http_svr() {
    let store = storage::store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    
    let log = warp::log::custom(|info| {
        let headers: & std::collections::HashMap<_, _> = &info
            .request_headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        eprintln!(
            "[{}] {}: {} {} {:?} from {} with \n{}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap_or_else(|| "unknown".parse().unwrap()),
            serde_json::to_string_pretty(headers).unwrap()
        );
    });

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "RUST-WEB=info".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query()) // Replace YourQueryStruct with your actual struct
        .and(store_filter.clone())
        .and_then(get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_questions request",
                method = %info.method(),
                path = %info.path(),
                remote_addr = ?info.remote_addr(),
                id = %uuid::Uuid::new_v4()
            )
        }));

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

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>()) // Assuming the ID is a String
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(crate::routes::handler::delete_question);
    
    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_answer);
     
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
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .with(log)
        .with(warp::trace::request())
        .recover(return_error);
    // Start the warp server on port 8083
    //println!("Starting server on http://localhost:8083");
    warp::serve(routes).run(([0, 0, 0, 0], 8083)).await;
}