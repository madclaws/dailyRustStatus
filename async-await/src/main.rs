use futures::prelude::*;
use futures::future::join_all;
use tokio::*;
use log::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn app() -> Result<()> {
    // println!("Starting..");
    // let resp1 = task::spawn(reqq(1));
    // let resp2 = task::spawn(reqq(2));

    // let _ = resp1.await??;
    // let _ = resp2.await??;

    let mut futures = vec![];
    for i in 1..=10 {
        let fut = get_and_analyse(i);
        futures.push(fut);
    }

    let results = join_all(futures).await;

    let mut total_ones = 0;
    let mut total_zeros = 0;

    for result in results {
        // `spawn_blocking` returns a `JoinResult` we need to unwrap first
        let ones_res: Result<(u64, u64)> = result;
        let (ones, zeros) = ones_res?;
        total_ones += ones;
        total_zeros += zeros;
    }

    println!("Ratio of ones/zeros: {:.02}",total_ones as f64 / total_zeros as f64);
    Ok(())
}

async fn get_and_analyse(n: usize) -> Result<(u64, u64)> {
    let response: reqwest::Response = reqwest::get(slowwly(1000)).await?;
    println!("Dataset {}", n);
    let txt = response.text().await?;
    let res = task::spawn_blocking(move || analyse(&txt)).await?;
    println!("Processed {}", n);
    Ok(res)
}

// Counting the number of ones and zeros in the bytes we get.
fn analyse(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();
    // Let's spend as much time as we can and count them in two passes
    let ones = txt.iter().fold(0u64, |acc, b: &u8| acc + b.count_ones() as u64);
    let zeros = txt.iter().fold(0u64, |acc, b: &u8| acc + b.count_zeros() as u64);
    (ones, zeros)
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


