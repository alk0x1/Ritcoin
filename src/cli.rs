use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde_json::json;
use crate::rpc::rpc;
use crate::wallets_2::Wallet;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
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
  Blockchain {
    #[arg(short, long)]
    create: bool,
    #[arg(short, long)]
    info: bool,
    #[arg(short, long, help = "Show all blocks in the blockchain")]
    show_blocks: bool,
   #[arg(short, long, help = "Show all transactions in the pool")]
    pool: bool,
  },
  Block {
    #[arg(short, long)]
    insert: bool,
    #[arg(short, long)]
    show_details: Option<String>,
    #[arg(short, long, help = "Show all transactions in a specific block")]
    list_transactions: Option<String>,
  },
  Transaction {
    #[arg(short, long)]
    create: bool,
    #[arg(short, long, help = "Public key of the sender")]
    from: Option<String>,
    #[arg(short, long, help = "Public key of the recipient")]
    to: Option<String>,
    #[arg(short, long, help = "Signature of the transaction")]
    signature: Option<String>,
    #[arg(long, help = "Show information for a specific transaction")]
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
      Commands::Blockchain { create, info, show_blocks, pool} => handle_blockchain_commands(*create, *info, *show_blocks, *pool).await, // Start the blockchain
      Commands::Block { insert, show_details, list_transactions } => handle_block_commands(*insert, show_details, list_transactions).await,
      Commands::Wallet { list, create, show, name } => handle_wallet_commands(*list, *create, *show, name).await,
      Commands::Transaction { create , from, to, signature, show} => handle_transactions_commands(*create, from, to, signature.clone()).await
    }
  }
}

// handlers
async fn handle_blockchain_commands(create: bool, info: bool, show_blocks: bool, pool: bool) {
  if create {
    rpc();
  }
  if show_blocks {
    show_all_block_hashes().await;  
  }
  if info {
    get_blockchain_data().await;
  }
  if pool {
    show_transactions_pool().await;
  }
}

async fn handle_block_commands(insert: bool, show_details: &Option<String>, list_transactions: &Option<String>) {
  if let Some(list_transactions) = list_transactions {
    show_transactions_in_a_block(list_transactions).await;
  } 
 if insert {
    insert_new_block().await;
  }
  if let Some(index) = show_details {
    show_block_info(&index).await;
  }
}

async fn handle_wallet_commands(list: bool, create: bool, show: bool, name: &Option<String>) {
  if list {
    // Intended for listing all wallets
    // show_all_block_hashes().await;
  } else if create {
    match name {
      Some(name) => {
        create_wallet(name).await;
        println!("created wallet in {}", name);
      },
      None => println!("Expected flag -n <wallet_name>")
    }
  } else if show {
    if let Some(wallet_name) = name {
      // Placeholder for showing specific wallet information
      println!("Showing information for wallet: {}", wallet_name);
    }
  }
}

async fn handle_transactions_commands(create: bool, from: &Option<String>, to: &Option<String>, signature: Option<String>) {
  if create {
    match from {
      Some(addr_from) => match to {
        Some(addr_to) => {
          // println!("create: {}, from: {}, to: {}", create, addr_from, addr_to);
          create_transaction(addr_from, addr_to).await
        },
        None => println!("Please provide the receiver address")
      }
      None => println!("Please provide the sender address")
    }
  }
}

// methods
async fn get_blockchain_data() {
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:3030")
                    .json(&json!({
                        "jsonrpc": "2.0",
                        "method": "get_blockchain_data",
                        "params": [],
                        "id": 5
                    }))
                    .send()
                    .await
                    .expect("Failed to send request");

    let response = res.text().await.expect("Failed to read response");

    // Assuming the response is in JSON format and has a "result" field with the data
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&response) {
        if let Some(result) = val["result"].as_str() {
            println!("Blockchain data:\n{}", result);
        } else {
            println!("Error retrieving blockchain data.");
        }
    } else {
        println!("Failed to parse response.");
    }
}

async fn show_all_block_hashes() {
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
    .json(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": "show_all_block_hashes",
        "params": [],
        "id": 2
    }))
    .send()
    .await
    .expect("Failed to send request");

  let response_text = res.text().await.expect("Failed to read response");
  let response_json: serde_json::Value = serde_json::from_str(&response_text)
    .expect("Failed to parse JSON");

  // Extract and print the "result" field correctly
  if let Some(result) = response_json["result"].as_str() {
    // Split the result into lines and print each line separately
    for line in result.split("\\n") {
      println!("{}", line);
    }
  } else {
    println!("No result found in response.");
  }
}

async fn show_transactions_pool() {
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
    .json(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": "show_transactions_pool",
        "params": [],
        "id": 1
    }))
    .send()
    .await
    .expect("Failed to send request");

  let response_text = res.text().await.expect("Failed to read response");
  let response_json: serde_json::Value = serde_json::from_str(&response_text)
    .expect("Failed to parse JSON");

  if let Some(result) = response_json["result"].as_array() {
    for transaction in result {
      println!("{}", serde_json::to_string_pretty(transaction).unwrap());
    }
  } else {
    println!("No transactions found in the pool.");
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

async fn show_block_info(index: &String) {
  // RPC call to show specific block information
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
                  .json(&json!({
                      "jsonrpc": "2.0",
                      "method": "show_block_info",
                      "params": [index],
                      "id": index
                  }))
                  .send()
                  .await
                  .expect("Failed to send request");

  let response = res.text().await.expect("Failed to read response");
  println!("{}", response);
}

async fn create_wallet(filename: &str) {
  let wallet = Wallet::new();
  wallet.save(&filename);
}

async fn create_transaction(from: &String, to: &String) {
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
                    .json(&json!({
                        "jsonrpc": "2.0",
                        "method": "insert_transaction_in_pool",
                        "params": [from, to],
                        "id": 4  // Ensure a unique ID for each request
                    }))
                    .send()
                    .await
                    .expect("Failed to send request");

    let response = res.text().await.expect("Failed to read response");
    println!("Transaction response: {}", response);
}

async fn show_transactions_in_a_block(block_identifier: &str) {
  let client = reqwest::Client::new();
  let res = client.post("http://127.0.0.1:3030")
    .json(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": "show_transactions_in_a_block",
        "params": [block_identifier],
        "id": 3
    }))
    .send()
    .await
    .expect("Failed to send request");

  let response_text = res.text().await.expect("Failed to read response");
  let response_json: serde_json::Value = serde_json::from_str(&response_text)
      .expect("Failed to parse JSON");

  // Extract and print the "result" field correctly
  if let Some(result) = response_json["result"].as_str() {
    // Split the result into lines and print each line separately
    for line in result.split("\\n") {
        println!("{}", line);
    }
  } else {
    println!("No result found in response.");
  }
}