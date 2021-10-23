use regex::Regex;
use solana_sdk::signature::{Keypair, Signer};
use std::time::Instant;

pub fn gen_acc(target: &str, is_regex: bool) {
    let mut pubkey_string = String::from("");
    let mut is_matched = false;
    let mut count = 0;
    let now = Instant::now();
    let mut keypair = Keypair::new();

    while !is_matched {
        // New Keypair
        keypair = Keypair::new();

        // Public key string 32
        pubkey_string = keypair.pubkey().to_string();

        // Matched?
        if is_regex {
            let re = Regex::new(target).unwrap();
            is_matched = re.is_match(&pubkey_string.to_owned());
        } else {
            is_matched = pubkey_string.starts_with(target);
        }

        // Log progress
        print!("\r");
        print!(
            "generated:{:?} ⚡️{} acc/s",
            count,
            count / (1 + now.elapsed().as_secs())
        );

        // Next
        count += 1;
    }

    // Keypair for write to disk = 32+32
    let keypair_bytes = keypair.to_bytes();
    let keypair_serialize = serde_json::to_string(&keypair_bytes.to_vec()).unwrap();
    // println!("{:?}", serialized);

    // Secret key string 32
    // let keypair_secret = keypair.secret();
    // let keypair_bytes = keypair_secret.to_bytes();
    // let secret_string = serde_json::to_string(&keypair_bytes.to_vec()).unwrap();
    // println!("{:?}", secret_string);
    let secret_string = keypair.to_base58_string();

    println!("\n------------------------------------------------");
    println!("pubkey: {}", pubkey_string);
    println!("secret: {}", secret_string);
    println!("keypair: {}", keypair_serialize);
    println!("------------------------------------------------");

    // Test
    // let keypair_test = Keypair::from_base58_string(&keypair_base58);
    // println!("\ntest pubkey: {}", keypair_test.pubkey().to_string());
}

pub fn gen_pubkey(target: &str, is_regex: bool) {
    let mut pubkey_string = String::from("");
    let mut is_matched = false;
    let mut count = 0;
    let now = Instant::now();

    while !is_matched {
        // Create new keypair
        let pubkey = solana_sdk::pubkey::new_rand();
        pubkey_string = pubkey.to_string();

        // Matched?
        if is_regex {
            let re = Regex::new(target).unwrap();
            is_matched = re.is_match(&pubkey_string.to_owned());
        } else {
            is_matched = pubkey_string.starts_with(target);
        }

        count += 1;

        print!("\r");
        print!(
            "generated:{:?} ⚡️{} acc/s",
            count,
            count / (1 + now.elapsed().as_secs())
        );
    }

    println!("\n------------------------------------------------");
    println!("pubkey = {}", pubkey_string);
    println!("------------------------------------------------");
}

fn main() {
    // gen_pubkey(r"^avar.\w+", true);
    // gen_pubkey(r"foo", false);
    gen_acc(r"test", false);
}
