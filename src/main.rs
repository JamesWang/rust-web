#![warn(clippy::all)]

mod minimal_http;
mod routes;
mod storage;
mod types;
mod utils;

use minimal_http::http_call::http_call;
use minimal_http::minimal_main::minimal_http_svr;
use types::question::{Question, QuestionId};

use handle_errors::return_error;

#[tokio::main]
async fn main() {
    //env_logger::init(); // Initialize the logger
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("Failed to initialize logger: {}", e);
    }
    
    log::error!("This is an error message");
    log::warn!("This is a warning message");
    log::info!("This is an info message");
    log::debug!("This is a debug message");
    log::trace!("This is a trace message");
    // Start the minimal HTTP server
    // minimal_http_svr().await;
    // The server will run indefinitely, handling requests as they come in.

    /* let question = Question::new(
           QuestionId::new("1".to_string()),
           "First Question".to_string(),
           "Content of question".to_string(),
           Some(vec!["faq".to_string()]),
       );
       println!("{}", question);
    */
    // Call the HTTP function
    /*     match http_call().await {
           Ok(_) => println!("HTTP call succeeded"),
           Err(e) => eprintln!("HTTP call failed: {}", e),
       }
    */

    // Start the minimal HTTP server
    minimal_http_svr().await;
}
