use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;



fn main() {
    dotenv().ok();

    // The public key as a string
    let public_key_str = "2AF3dYxFnytyg2EfDsCSnvGYGuBcy2BTmSNaC9P4g6s2";

    // Convert the string into a Pubkey type
    let public_key = Pubkey::from_str(public_key_str)
                                .expect("Invalid public key");
    

    // Create a connection to the Solana devnet
    let rpc_url = "https://api.devnet.solana.com"; // Cluster URL
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), 
                                                    solana_sdk::commitment_config::CommitmentConfig::confirmed());

    println!("Connection was established");

    // Fetch the balance
    let balance = client.get_balance(&public_key).expect("Failed to get balance");

    // Print the balance in SOL
    println!("Balance = {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);
}
