// Pay to Public Key(P2PK)
// After the unlocking script pushes a signature onto the stack, this script checks if the signature is valid, if so the outputs can be spent. Operations involved with this script include the following;
// OP_DATA_X - adds the recipient's compressed or uncompressed public key to the stack.
// OP_CHECKSIG - compares the public key at the top of the sack with the signature below it on the stack.
 
pub fn locking_script() {
  let pub_key = ""; // when doublehashed will be equal the wallet
  let test_public_key = "15t7x2RYTpxNsXZAwfLhz9ZCHTLsF7vBio";
  let test_private_key = "KymrvyXu1hSnJyH5FjY1xvmWUJYRPQ7bZfBTkrXs5wUBYFWGZiww";

}

pub fn unlocking_script() {
  
}

struct Stack<T> {
  stack: Vec<T>,
}