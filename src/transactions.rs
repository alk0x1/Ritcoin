use rand::Rng;
use sha2::digest::{generic_array::GenericArray, typenum::{UInt, UTerm, bit::{B1, B0}}};
extern crate hex;
use crate::utils;


#[derive(Debug)]
pub struct Transaction {
  pub version: usize,
  pub input_counter: usize,
  pub signature: String,
  // inputs: Vec<Inputs>,
  // outputs: Vec<Outputs>,
  // locktime: Date
}

pub struct Output {
  amount: usize,
  locking_script_size: usize,
  // locking_script: nao_sei_o_tipo_aqui_n
}

pub struct Input {
  transaction_hash: String,
  output_index: usize,
  unlocking_script_size: usize,
  // unlocking_script: nao_sei_o_tipo_aqui_tbm,
  sequence_number: usize
}

impl Transaction {
  pub fn new(version: usize, input_counter: usize, signature: String) -> Self {
    Transaction {
      version,
      input_counter,
      signature
    }
  }
  
  pub fn new_pseudo_hash() -> String {
    let random_number = &[rand::thread_rng().gen()];
		let random_number_converted= std::str::from_utf8(random_number);
		
		let random_number_result = match random_number_converted {
			Ok(random_number_converted) => random_number_converted,
			Err(e) => "error"
		};
		
    if random_number_result != "error" {
      let pseudo_hash = utils::hash(random_number_result);
      let hex_pseudo_hash= hex::encode(pseudo_hash);

      return hex_pseudo_hash;
		} else {
      return Self::new_pseudo_hash();
    }
  }

}