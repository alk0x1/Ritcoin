use sha2::{Digest, Sha256, digest::{generic_array::GenericArray, typenum::{UInt, UTerm, bit::{B1, B0}}}};

pub fn hash(data: &str) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> {
  let mut hasher = Sha256::new();
  hasher.update(data);
  return hasher.finalize();
}

pub fn concat_strings(string1: String, string2: String) -> String {
  let mut owned_string: String = string1.to_owned();
  let another_owned_string: String = string2.to_owned();
  owned_string.push_str(&another_owned_string);

  return owned_string;
}

pub fn validated_hash(hash: String, difficulty: usize, prefix: String) -> bool {
  let check = prefix.repeat(difficulty);
  println!("hash.starts: {}", hash);
  return hash.starts_with(&check);
}
// workaround to get element from string vector without ownership problems
pub fn get_string_vec_content(vec: Vec<String>, index: usize) -> String {
  let args = vec;
  let ref dir = **&args[index];
  return String::from(dir);
}

pub fn print_menu() {
  println!("-------------------------------------");
	println!("1. Create transaction");
	println!("2. Create block with the first 15 transactions");
	println!("9. Show this menu again");
	println!("-------------------------------------");
}