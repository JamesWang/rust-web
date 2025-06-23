use std::collections::HashMap;

pub async fn http_call() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
        //so the runtime will handle the task 
        // in the background and fill our content 
        // variable when the file is read.
    println!("{:#?}", response);
    Ok(())
}