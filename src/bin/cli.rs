use ritcoin::cli;

#[tokio::main]
async fn main() {
  cli::spawn().await;
}
