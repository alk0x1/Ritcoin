use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde_json::json;
use crate::rpc::rpc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  #[arg(short, long)]
  name: Option<String>,

  #[arg(short, long, value_name = "FILE")]
  config: Option<PathBuf>,

  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  Start,
  Block {
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    insert: bool,
    #[arg(short, long)]
    show: Option<String>,
  },
  Wallet {
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    create: bool,
    #[arg(short, long)]
    show: bool,
    #[arg(short, long)]
    name: Option<String>,
  },
}

pub async fn spawn() {
  let cli = Cli::parse();

  if let Some(command) = &cli.command {
    match command {
      Commands::Start => rpc(),
      Commands::Block { insert, list, show } => handle_block_commands(*insert, *list, show).await,
      Commands::Wallet { list, create, show, name } => handle_wallet_commands(*list, *create, *show, name).await,
    }
  }
}

async fn insert_new_block() {
  // Keeps the RPC call to insert a new block
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
                  .json(&json!({
                      "jsonrpc": "2.0",
                      "method": "insert_new_block",
                      "params": [],
                      "id": 1
                  }))
                  .send()
                  .await
                  .expect("Failed to send request");

  let response = res.text().await.expect("Failed to read response");
  println!("{}", response);
}

async fn handle_block_commands(insert: bool, list: bool, show: &Option<String>) {
  if list {
    show_all_block_hashes().await;
  } else if insert {
    insert_new_block().await;
  } else if let Some(index) = show {
    show_block_info(index).await;
  }
}

async fn handle_wallet_commands(list: bool, create: bool, show: bool, name: &Option<String>) {
  if list {
    // Intended for listing all wallets
    show_all_block_hashes().await;
  } else if create {
    create_wallet().await;
  } else if show {
    if let Some(wallet_name) = name {
      // Placeholder for showing specific wallet information
      println!("Showing information for wallet: {}", wallet_name);
    }
  }
}

async fn show_all_block_hashes() {
  // RPC call to list all block hashes
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
                  .json(&json!({
                      "jsonrpc": "2.0",
                      "method": "show_all_block_hashes",
                      "params": [],
                      "id": 2
                  }))
                  .send()
                  .await
                  .expect("Failed to send request");

  let response = res.text().await.expect("Failed to read response");
  println!("{}", response);
}

async fn show_block_info(index: &String) {
  // RPC call to show specific block information
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
                  .json(&json!({
                      "jsonrpc": "2.0",
                      "method": "show_block_info",
                      "params": [index],
                      "id": 3
                  }))
                  .send()
                  .await
                  .expect("Failed to send request");

  let response = res.text().await.expect("Failed to read response");
  println!("{}", response);
}

async fn create_wallet() {
    let client = reqwest::Client::new();
    let filename = "my_new_wallet.json";  // This should be dinamic.

    let res = client.post("http://127.0.0.1:3030")
                    .json(&json!({
                        "jsonrpc": "2.0",
                        "method": "create_wallet",
                        "params": [filename],
                        "id": 1
                    }))
                    .send()
                    .await
                    .expect("Failed to send request");

    // Printing out the response directly, similar to how it's done in insert_new_block.
    let response = res.text().await.expect("Failed to read response");
    println!("{}", response);
}
