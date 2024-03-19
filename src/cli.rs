use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde_json::json;
use crate::rpc::rpc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  /// Optional name to operate on
  name: Option<String>,

  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  config: Option<PathBuf>,

  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// run a node
  Start,
  /// insert block
  Block {
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    insert: bool,
    #[arg(short, long)]
    show: Option<String>
  },
  /// manage wallets
  Wallet {
    /// list wallets in ./wallets.json file
    #[arg(short, long)]
    list: bool,
    /// create a new wallet
    #[arg(short, long)]
    create : bool,
    /// Displays information about a specific wallet.
    #[arg(short, long)]
    show: bool,  
    /// Specifies the name of the wallet for the 'show' operation.
    #[arg(short, long)]
    name: Option<String>,
  }
}

pub async fn spawn() {
  let cli = Cli::parse();
  // You can check the value provided by positional arguments, or option arguments
  if let Some(name) = cli.name.as_deref() {
    println!("Value for name: {}", name);
  }

  if let Some(config_path) = cli.config.as_deref() {
    println!("Value for config: {}", config_path.display());
  }

  // You can see how many times a particular flag or argument occurred
  // Note, only flags can have multiple occurrences
  match cli.debug {
    0 => println!("Debug mode is off"),
    1 => println!("Debug mode is kind of on"),
    2 => println!("Debug mode is on"),
    _ => println!("Don't be crazy"),
  }

  // You can check for the existence of subcommands, and if found use their
  // matches just as you would the top level cmd
  if let Some(command) = &cli.command {
    match command {
      Commands::Start => {  
        rpc()
      }
      Commands::Block {insert, list, show} => {
        handle_block_commands(*insert, *list, show).await
      },
      Commands::Wallet { list, create, show, name } => {
        if *list {
          println!("Listing all wallets...");
          show_all_block_hashes().await
        }
        if *create {
          println!("Creating a new wallet...");
          // Implement the logic to create a new wallet.
        }
        if *show {
          match name {
            Some(wallet_name) => {
              println!("Showing information for wallet: {}", wallet_name);
              // Implement the logic to show wallet information.
            }
            None => {
              println!("Please specify a wallet name to show its information.");
            }
          }
        }
      }
    }
  }
}

async fn insert_new_block() {
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

  println!("Response: {:?}", res.text().await.expect("Failed to read response"));
}

async fn handle_block_commands(insert: bool, list: bool, show: &Option<String>) {
  if list {
    show_all_block_hashes().await;
  }
  else if insert {
    insert_new_block().await;
  }
  else if let Some(index) = show {
    show_block_info(index).await;
  }
  else {
    println!("Please provide one of the following options: insert, list, show");
  }
}

async fn handle_transactions_commands() {}

async fn handle_wallet_commands() {}


async fn get_last_block_hash() {
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
                  .json(&json!({
                      "jsonrpc": "2.0",
                      "method": "get_last_block_hash",
                      "params": [],
                      "id": 1
                  }))
                  .send()
                  .await
                  .expect("Failed to send request");

  let response = res.text().await.expect("Failed to read response");
  println!("Last block hash: {}", response);
}

async fn show_all_block_hashes() {
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
  println!("All block hashes: {}", response);
}

async fn show_block_info(index: &String) {
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
  println!("Block info for index {}: {}", index, response);
}

