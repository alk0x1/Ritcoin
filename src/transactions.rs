use rand::Rng;
use sha2::digest::{generic_array::GenericArray, typenum::{UInt, UTerm, bit::{B1, B0}}};
extern crate hex;


use crate::utils;

pub struct Transactions {
  publick_key: String,
  hash: String,
  signature: String
}

impl Transactions {
  pub fn new() -> String {
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
      return Self::new();
    }
  }
}