use rsa::pkcs1v15;
use rsa::pkcs8::der::zeroize::Zeroizing;
use rsa::pkcs8::{EncodePublicKey, DecodePublicKey, EncodePrivateKey, DecodePrivateKey};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPublicKey};

pub fn create_wallet() {
  let mut rng = rand::thread_rng();
  let bits = 2048;
  let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
  let pub_key = RsaPublicKey::from(&priv_key);


  let (encoded_public_key, encoded_private_key) = encode_keys_pkcs8(pub_key.clone(), priv_key.clone());
  let (decoded_public_key, decoded_private_key) = decode_keys_pkcs8(encoded_public_key.clone(), encoded_private_key.clone());

  println!("Public Key: {:?}\nSecret Key: {:?}", encoded_public_key, encoded_private_key.as_str());

  assert_eq!(pub_key, decoded_public_key);
  assert_eq!(priv_key, decoded_private_key);
  println!("passou")
}

pub fn encode_keys_pkcs8(pubkey: RsaPublicKey, privkey: RsaPrivateKey) -> (String, Zeroizing<String>) {
  let public_key = pubkey.to_public_key_pem(rsa::pkcs8::LineEnding::CR).unwrap();
  let private_key = privkey.to_pkcs8_pem(rsa::pkcs8::LineEnding::CR).unwrap();

  (public_key, private_key)
}

pub fn decode_keys_pkcs8(pubpem: String, privpem: Zeroizing<String>) -> (RsaPublicKey, RsaPrivateKey) {
  let public_key = RsaPublicKey::from_public_key_pem(&pubpem).unwrap();
  let private_key = RsaPrivateKey::from_pkcs8_pem(&privpem).unwrap();

  (public_key, private_key)
}