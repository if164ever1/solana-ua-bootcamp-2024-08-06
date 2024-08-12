
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use solana_sdk::{signature::{Keypair, Signer}, signer::keypair};


const ERROR_MSG: &str = "SECRET_KEY is not defined in the .env file";
const FAILED_SECRET_KEY: &str = "Failed to parse SECRET_KEY";
const INVALID_SECRET_KEY: &str = "Invalid secret key";

fn main() {
    dotenv().ok();

    let key = "SECRET_KEY";
    let secret_key = env::var(key).expect(ERROR_MSG);

    let key_as_array: Vec<u8> = serde_json::from_str(&secret_key).expect(FAILED_SECRET_KEY);
    let keypair = Keypair::from_bytes(&key_as_array).expect(INVALID_SECRET_KEY);

    println!("Secret key: {:?}", keypair.secret());
    println!("Secret key: {:?}", keypair.pubkey());
}