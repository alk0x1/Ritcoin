use crate::transactions;

#[derive(Debug)]
struct Header {
	version: i32,
	// previous_hash: String,
	// timestamp: String,
	// merkle_root: String,
	// difficulty: i32
}
#[derive(Debug)]
pub struct Block {
	header: Header,
	// size: f32,
	transactions_counter: i16,
	transactions: Vec<String>
}
// Modify that later to have only a new() method and 
// create the genesis block in Blockchain class
impl Block {
	pub fn new(version: i32, transactions_counter: i16, transactions: Vec<String>) -> Self {
		let header = Header {
			version: 1
		};

		Block {
			header,
			transactions_counter,
			transactions
		}
	}

	// pub fn create_genesis_block() -> Block {
	// 	let genesis_block_header: Header = Header { 
	// 		version: 1,
	// 		// previous_hash: String::from("00"),
	// 		// timestamp: String::from("00"),
	// 		// merkle_root: String::from("00"),
	// 		// difficulty: 1,
	// 	};
	// 	let genesis_block: Block = Block { 
	// 		header: genesis_block_header,
	// 		// size: 0.0,
	// 		transactions_counter: 0,
	// 		// transactions: 0
	// 	};
	// 	return genesis_block;
	// }
}
