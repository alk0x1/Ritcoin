use sha2::{Digest, Sha256, digest::{generic_array::GenericArray, typenum::{UInt, UTerm, bit::{B1, B0}}}};

pub fn hash(data: &str) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> {
  let mut hasher = Sha256::new();
  hasher.update(data);
  return hasher.finalize();
}

pub fn print_menu() {
  println!("-------------------------------------");
	println!("1. Create transaction");
	println!("2. Create block with the first 15 transactions");
	println!("9. Show this menu again");
	println!("-------------------------------------");
}