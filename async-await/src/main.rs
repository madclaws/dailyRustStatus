use futures::prelude::*;
use tokio::*;
use log::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn app() -> Result<()> {
    println!("Starting..");
    let resp1 = task::spawn(reqq(1));
    let resp2 = task::spawn(reqq(2));

    let _ = resp1.await??;
    let _ = resp2.await??;


    Ok(())
}

fn main() {
    println!("Grokking async await");
    let start = std::time::Instant::now();

    let mut runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(app()) {
        Ok(_) => println!("Done"),
        Err(e) => println!("An error occured: {}", e)
    }
}

fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk", 
        delay_ms,
        );
        reqwest::Url::parse(&url).unwrap()
}

async fn reqq(n: usize) -> Result<()> {
    reqwest::get(slowwly(1000)).await?;
    println!("Got response {}", n);
    Ok(())
}


