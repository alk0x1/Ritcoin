use std::io::stdin;

use sha2::{Digest, Sha256, digest::{generic_array::GenericArray, typenum::{UInt, UTerm, bit::{B1, B0}}}};
use crate::block::Header;

pub fn hash(data: &str) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> {
  let mut hasher = Sha256::new();
  hasher.update(data);
  return hasher.finalize();
}

pub fn hash_block(block_header: &Header) -> String {
  let nonce = block_header.nonce.to_string();
  let previous_hash = block_header.previous_hash.clone();
  let version = block_header.version.to_string();

  let mut to_be_hashed = concat_strings(version, previous_hash);
  to_be_hashed = concat_strings(to_be_hashed, nonce);

  return hex::encode(hash(&to_be_hashed));
}

pub fn concat_strings(string1: String, string2: String) -> String {
  let mut owned_string: String = string1.to_owned();
  let another_owned_string: String = string2.to_owned();
  owned_string.push_str(&another_owned_string);

  return owned_string;
}

pub fn validated_hash(hash: String, difficulty: usize, prefix: String) -> bool {
  let check = prefix.repeat(difficulty);
  // println!("hash.starts: {}", hash);
  return hash.starts_with(&check);
}
// workaround to get element from string vector without ownership problems
// pub fn get_string_vec_content(vec: Vec<String>, index: usize) -> String {
//   let args = vec;
//   let ref dir = **&args[index];
//   return String::from(dir);
// }

pub fn print_menu() {
  println!("-------------------------------------");
	println!("1. Create transaction");
	println!("2. Insert a new test block in the blockchain");
	println!("3. Show all the blocks hashes");
	println!("4. Show all transactions signatures");
	println!("5. Create Ritcoin Wallet");
	println!("9. Show this menu again");
	println!("-------------------------------------");
}

pub fn mount_transaction_menu() {
  
}

// pub fn interact_with_menu() -> usize {
//   let mut option = String::new();
//   let mut parsed: usize = 00;
//   while option.trim_end() != String::from("00") {
//     option = String::from("");
//     stdin().read_line(&mut option).expect("failed to readline");
//     parsed = match option.trim_end().parse::<usize>() {
//       Ok(value) => value,
//       Err(_) => 00
//     };
//   }
//   parsed
// }