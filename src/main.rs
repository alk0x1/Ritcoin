use std::{io::stdin, option};

use transactions::Transaction;
mod block;
mod blockchain;
mod utils;
mod transactions;

fn main() {
	utils::print_menu();
	let mut option1 = String::new();
	let mut new_blockchain = blockchain::Blockchain::new();

	loop {
		if option1 == String::from("") {
			stdin().read_line(&mut option1).expect("failed to readline");
		}
		
		if option1.trim_end() == String::from("9") {
			utils::print_menu();
		}

		if option1.trim_end() == String::from("1") {
			let new_transaction = transactions::Transaction::new(1, 0, String::from("SIGNATURE_TEST"));
			new_blockchain.transactions_pool.push(new_transaction);

			println!("transaction: {:?}", new_blockchain.transactions_pool[new_blockchain.transactions_pool.len() - 1]);
		}
		
		if option1.trim_end() == String::from("2") {
			new_blockchain.insert_new_block();
		}
		
		if option1.trim_end() == String::from("3") {
			println!("Press the correspondent number to see more details or 00 to go back");
			new_blockchain.show_all_block_hashes();
			
			let mut option2 = String::new();
			while option2.trim_end() != String::from("00") {
				option2 = String::from("");
				stdin().read_line(&mut option2).expect("failed to readline");
				let parsed: usize = match option2.trim_end().parse::<usize>() {
					Ok(value) => value,
					Err(_) => 00
				};
				new_blockchain.show_block_info(parsed);
			}
		}
		if option1.trim_end() == String::from("4") {
		  println!("Press the correspondent number to see more details or 00 to go back");
			new_blockchain.show_all_transactions();
			
			let mut option2 = String::new();
			while option2.trim_end() != String::from("00") {
				option2 = String::from("");
				stdin().read_line(&mut option2).expect("failed to readline");
				let parsed: usize = match option2.trim_end().parse::<usize>() {
					Ok(value) => value,
					Err(_) => 00
				};
				new_blockchain.show_transaction_info(parsed);
			}
		}

		option1 = String::from("");
	}
}
