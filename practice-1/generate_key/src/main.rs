use solana_sdk::{signature::{Keypair}};
use base58::ToBase58;

fn generate_key_pair() -> (String, Vec<u8>){
    let keypair = Keypair::new();

    let public_key = keypair.to_bytes().to_base58();

    let private_key = keypair.to_bytes().to_vec();

    (public_key, private_key)
}


fn main() {
    let (public_key, private_key) = generate_key_pair();

    print!("Public key: {}", public_key);
    print!("Private key: {:?}", private_key);
}
