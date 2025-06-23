mod minimal_http;
mod models;
//use minimal_http::minimal_main::minimal_http_svr;
use models::question::{Question, QuestionId};
use minimal_http::http_call::http_call;

#[tokio::main]
async fn main() {
    // Start the minimal HTTP server
    // minimal_http_svr().await;
    // The server will run indefinitely, handling requests as they come in.

    let question = Question::new(
        QuestionId::new("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{}", question);

    // Call the HTTP function
    match http_call().await {
        Ok(_) => println!("HTTP call succeeded"),
        Err(e) => eprintln!("HTTP call failed: {}", e), 
    }
}
