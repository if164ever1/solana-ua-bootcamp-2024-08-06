use solana_sdk::{feature::from_account, signature::Keypair};
use base58::ToBase58;
use std::time::{Instant, Duration};

fn generate_key_pair() -> (String, Vec<u8>){
    let keypair = Keypair::new();

    let public_key = keypair.to_bytes().to_base58();

    let private_key = keypair.to_bytes().to_vec();

    (public_key, private_key)
}


fn main() {
    let key_word = "XX"; // Your desired prefix
    let mut found = false;
    let mut attempts = 0;

    let start_time = Instant::now();

    let mut last_update_time = Instant::now();

    while !found {
        let (public_key, private_key) = generate_key_pair();

        attempts += 1;

        let slice = &public_key[..key_word.len()]; // Match length of key_word

       

        if slice == key_word {
            found = true;
            println!("\nMatch found!");
            println!("Public key: {}", public_key);
            println!("Private key: {:?}", private_key);
        }

        // Print status every second or after a certain number of attempts
        if last_update_time.elapsed() >= Duration::from_secs(1) || attempts % 1000 == 0 {
            let duration = start_time.elapsed();
            println!(
                "Attempts: {} | Time elapsed: {:.2?} | Current Public Key: {}",
                attempts/1000, duration, public_key
            );
            last_update_time = Instant::now(); // Reset the update timer
        }
    }

    // Final output
    let total_duration = start_time.elapsed();
    println!("\nTotal Attempts: {}", attempts);
    println!("Total Time elapsed: {:.2?}", total_duration);
}
