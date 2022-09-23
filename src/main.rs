use std::io::{stdin,stdout,Write};

use rand::Rng;
use sha2::digest::{typenum::{bit::{B0, B1}, UTerm, UInt}, generic_array::GenericArray};
mod block;
mod blockchain;
mod utils;
mod transactions;


// put all transactions in a mem-pool as pending, miners will verify the transactions (in bitcoin model)
// then the validated transactions will be stored in a block as soon it is ready.

fn main() {
	utils::print_menu();
	let mut option = String::new();

	loop {
		let mut pseudo_transactions: Vec<String> = Vec::new();
		while pseudo_transactions.len() < 15 {
			stdin().read_line(&mut option).expect("failed to readline");
			if option.trim_end() == String::from("9") {
				utils::print_menu();
			}
	
			if option.trim_end() == String::from("1") {
				let new_transaction = transactions::Transactions::new();
				pseudo_transactions.push(new_transaction);
				println!("transaction: {:?}", pseudo_transactions[pseudo_transactions.len() - 1]);
			}
			
			option = String::from("");
		}
		
		// if option.trim_end() == String::from("2") {
		let new_block = block::Block::new(1, 15, pseudo_transactions);
		println!("Block: {:?}", new_block);
		// }
	}
}
