pub fn ex1_func() {
    let mut address = String::from("Street 1");
    add_postal_code(&mut address);

    println!("{}", address);
}

fn add_postal_code(address: &mut String) {
    address.push_str(", 12345 Kingston");
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt};

pub async fn example_http_call() -> Result<()> {
    let client = Client::new();
    let mut res = client.get("http://www.google.com".parse::<hyper::Uri>().unwrap()).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

   while let Some(chunk) = res.data().await {
        let chunk = chunk?;
        io::stdout().write_all(&chunk).await?;
    }
    println!("\nResponse body fully read.");
    Ok(())
}