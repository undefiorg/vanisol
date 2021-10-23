use regex::Regex;
use solana_sdk::signature::{Keypair, Signer};
use std::string::String;
use std::time::Instant;

pub fn gen_keypair(target: &str, is_regex: bool) -> (String, String, String) {
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
    let serialized_keypair = serde_json::to_string(&keypair_bytes.to_vec()).unwrap();
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
    println!("keypair: {}", serialized_keypair);
    println!("------------------------------------------------");

    // Test
    // let keypair_test = Keypair::from_base58_string(&keypair_base58);
    // println!("\ntest pubkey: {}", keypair_test.pubkey().to_string());
    (pubkey_string, secret_string, serialized_keypair)
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
    gen_keypair(r"test", false);
}

#[test]
fn test_gen_keypair() {
    let (pubkey_string, secret_string, serialized_keypair) = gen_keypair(r"z", false);
    assert_eq!(pubkey_string.starts_with("z"), true);

    assert_eq!(secret_string.chars().count(), 88);
    let keypair_test = Keypair::from_base58_string(&secret_string);
    assert_eq!(keypair_test.pubkey().to_string(), pubkey_string);

    let bytes: Vec<u8> = serde_json::from_str(&serialized_keypair).unwrap();
    let dalek_keypair = Keypair::from_bytes(&bytes);
    assert_eq!(dalek_keypair.unwrap().pubkey().to_string(), pubkey_string);
}
