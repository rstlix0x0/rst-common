use rst_common::with_tokio::*;

async fn hello_async() {
    println!("hello async")
}

#[tokio::main]
async fn main() {
    hello_async().await
}