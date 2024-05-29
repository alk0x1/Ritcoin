	
#[tokio::main]
async fn main() {
  ritcoin::cli::spawn().await;
	// utils::print_menu();
	// let mut option1 = String::new();
	// let mut new_blockchain = blockchain::Blockchain::new();

	// loop {
	// 	if option1 == String::from("") {
	// 		stdin().read_line(&mut option1).expect("failed to readline");
	// 	}
		
	// 	if option1.trim_end() == String::from("9") {
	// 		utils::print_menu();
	// 	}

	// 	if option1.trim_end() == String::from("1") {
	// 		println!("");
	// 		mount_transaction_menu(&mut new_blockchain);
	// 		// let new_transaction = transactions::Transaction::new(1, 0, String::from("SIGNATURE_TEST"));
	// 		// new_blockchain.transactions_pool.push(new_transaction);

	// 		// println!("transaction: {:?}", new_blockchain.transactions_pool[new_blockchain.transactions_pool.len() - 1]);
	// 	}
		
	// 	if option1.trim_end() == String::from("2") {
	// 		new_blockchain.insert_new_block();
	// 	}
		
	// 	if option1.trim_end() == String::from("3") {
	// 		println!("Press the correspondent number to see more details or 00 to go back");
	// 		new_blockchain.show_all_block_hashes();
			
	// 		let mut option2 = String::new();
	// 		while option2.trim_end() != String::from("00") {
	// 			option2 = String::from("");
	// 			stdin().read_line(&mut option2).expect("failed to readline");
	// 			let parsed: usize = match option2.trim_end().parse::<usize>() {
	// 				Ok(value) => value,
	// 				Err(_) => 00
	// 			};
	// 			new_blockchain.show_block_info(parsed);
	// 		}
	// 	}
	// 	if option1.trim_end() == String::from("4") {
	// 	  println!("Press the correspondent number to see more details or 00 to go back");
	// 		new_blockchain.show_all_transactions();
			
	// 		let mut option2 = String::new();
	// 		while option2.trim_end() != String::from("00") {
	// 			option2 = String::from("");
	// 			stdin().read_line(&mut option2).expect("failed to readline");
	// 			let parsed: usize = match option2.trim_end().parse::<usize>() {
	// 				Ok(value) => value,
	// 				Err(_) => 00
	// 			};
	// 			new_blockchain.show_transaction_info(parsed);
	// 		}
	// 	}
	// 	if option1.trim_end() == String::from("5") {
	// 			mount_wallet_menu()
	// 	}

	// 	option1 = String::from("");
	// }
}


// fn mount_transaction_menu(blockchain: &mut Blockchain) {
//  // Initialize an empty string to hold user input
//     let mut sender = String::new();
//     let mut recipient = String::new();
//     let mut amount = String::new();
//     let mut fee = String::new();
//     let txid = Transaction::new_pseudo_hash();  // Generate a pseudo-unique ID for this new transaction

//     // Prompt the user for the sender's address
//     print!("Enter sender address: ");
//     stdout().flush().unwrap();  // Ensure the prompt is displayed immediately
//     stdin().read_line(&mut sender).expect("Failed to read line");
//     sender = sender.trim().to_string();  // Trim any trailing newline characters

//     // Prompt the user for the recipient's address
//     print!("Enter recipient address: ");
//     stdout().flush().unwrap();
//     stdin().read_line(&mut recipient).expect("Failed to read line");
//     recipient = recipient.trim().to_string();

//     // Prompt the user for the transaction amount
//     print!("Enter transaction amount: ");
//     stdout().flush().unwrap();
//     stdin().read_line(&mut amount).expect("Failed to read line");
//     amount = amount.trim().to_string();

//     // Prompt the user for the transaction fee
//     print!("Enter transaction fee: ");
//     stdout().flush().unwrap();
//     stdin().read_line(&mut fee).expect("Failed to read line");
//     fee = fee.trim().to_string();

//     // Output the entered details (in a real application, you'd create and process the transaction here)
//     println!("\nTransaction details:");
//     println!("Sender: {}", sender);
//     println!("Recipient: {}", recipient);
//     println!("Amount: {}", amount);
//     println!("Fee: {}", fee);

// 		let inputs = vec![Input {
// 			txid: "input_txid".to_string(),
// 			vout: 0,
// 			script_sig: "signature".to_string()
// 		}];

// 		let outputs = vec![UTXO {
// 			txid: txid.clone(),
// 			index: 0,  // Output index 0 for this transaction
// 			value: 100,  // Example value
// 			script_pubkey: "recipient_address".to_string(),  // Placeholder recipient
//     }];

//     // Here you would typically construct the transaction object and process it
//     let transaction = Transaction::new(txid, inputs, outputs);
// 		println!("Created transaction: {:?}", transaction);

// 		// Insert in pool
// 		blockchain.insert_transaction_in_pool(transaction);
// }


// fn mount_wallet_menu() {
//   // Initialize an empty string to hold user input and a variable for selection
//   let mut selection = String::new();

//   println!("Wallet Management");
//   println!("1. Create new wallet");
//   println!("2. Display wallet public key");
//   println!("3. Export wallet private key");
//   println!("4. Import wallet using private key");
//   println!("Please enter your choice:");

//   stdout().flush().unwrap();  // Ensure the prompt is displayed immediately
//   stdin().read_line(&mut selection).expect("Failed to read line");
//   let selection = selection.trim().to_string();  // Trim any trailing newline characters

//   match selection.as_str() {
//     "1" => {
//       // Create a new wallet instance
//       let wallet = Wallet::new();
//       println!("New wallet created.");
//       println!("Public Key: {:?}", wallet.public_key);
//     },
//     "2" => {
//       // Assume we need the wallet instance which should be managed elsewhere, not within this method.
//       // In a real scenario, you would track wallet instances within an application scope, user session, or similar.
//       // Here we just create a new one for demonstration purposes.
//       let wallet = Wallet::new();
//       println!("Public Key: {:?}", wallet.public_key);
//     },
//     "3" => {
//       // Exporting the wallet's private key
//       let wallet = Wallet::new(); // In practice, you would use an existing wallet instance.
//       println!("WARNING: Be careful with your private key!");
//       println!("Private Key: {:?}", wallet.secret_key);
//     },
//     "4" => {
//       // Importing a wallet requires entering a secret key
//       println!("Enter your secret key (hex format):");
//       let mut secret_key_hex = String::new();
//       stdout().flush().unwrap();
//       stdin().read_line(&mut secret_key_hex).expect("Failed to read line");
//       let secret_key_hex = secret_key_hex.trim().to_string();

//       // The following assumes the Wallet can be instantiated with a hex private key
//       // You'd need a corresponding Wallet::import_from_hex or similar method
//       // For demonstration, we call new and don't actually import:
//       // let wallet = Wallet::import_from_hex(&secret_key_hex);
//       let wallet = Wallet::new(); // Placeholder for actual import
//       println!("Wallet imported. Public Key: {:?}", wallet.public_key);
//     },
//     _ => println!("Invalid selection."),
//   }
// }
