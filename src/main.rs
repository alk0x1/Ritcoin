use std::io::stdin;

use transactions::Transaction;
mod block;
mod blockchain;
mod utils;
mod transactions;


// put all transactions in a mem-pool as pending
// broadcast to all nodes that will validate and put new transactions into a block
// start to solve the puzzle

// 1) New transactions are broadcast to all nodes.
// 2) Each node collects new transactions into a block.
// 3) Each node works on finding a difficult proof-of-work for its block.
// 4) When a node finds a proof-of-work, it broadcasts the block to all nodes.
// 5) Nodes accept the block only if all transactions in it are valid and not already spent.
// 6) Nodes express their acceptance of the block by working on creating the next block in the
// chain, using the hash of the accepted block as the previous hash.


fn main() {
	utils::print_menu();
	let mut option = String::new();
	let mut new_blockchain = blockchain::Blockchain::new();

	loop {
		stdin().read_line(&mut option).expect("failed to readline");
		
		if option.trim_end() == String::from("9") {
			utils::print_menu();
		}

		if option.trim_end() == String::from("1") {
			let new_transaction = transactions::Transaction::new(1, 0, String::from("SIGNATURE_TEST"));
			new_blockchain.transactions_pool.push(new_transaction);

			println!("transaction: {:?}", new_blockchain.transactions_pool[new_blockchain.transactions_pool.len() - 1]);
		}
		
		if option.trim_end() == String::from("2") {
			new_blockchain.insert_new_block();
		}
		
		if option.trim_end() == String::from("3") {
			new_blockchain.show_all_block_hashes();
		}

		if option.trim_end() == String::from("4") {
			new_blockchain.show_all_transactions();
		}

		option = String::from("");

		// let new_block = block::Block::new(1, 15, pseudo_transactions);
	}
}
