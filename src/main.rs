use anchor_client::solana_sdk::signature::{Keypair};
use regex::Regex;
use std::time::{Instant};

pub fn gen_acc(target: &str) {
  let mut pubkey = String::from("");
  let mut secret = String::from("");
  let mut is_matched = false;
  let mut count = 0;
  let now = Instant::now();

  while !is_matched {
    // Create new keypair
    let account = Keypair::new();
    pubkey = account.to_base58_string();
    secret = bs58::encode(account.secret().to_bytes()).into_string();

    // Matched?
    let re = Regex::new(target).unwrap();
    is_matched = re.is_match(&pubkey.to_owned());
    count+=1;

    print!("\r");
    print!("generated:{:?} ⚡️{} acc/s", count, count/(1+now.elapsed().as_secs()));
  }

  println!("\nnew account address = {}", pubkey);
  println!("secret: {}",secret);
}

fn gen_pubkey(target: &str) {
    let mut pubkey_string = String::from("");
    let mut is_matched = false;
    let mut count = 0;
    let now = Instant::now();
  
    while !is_matched {
      // Create new keypair
      let pubkey = anchor_client::solana_sdk::pubkey::new_rand();
      pubkey_string = pubkey.to_string();

      // Matched?
      let re = Regex::new(target).unwrap();
      is_matched = re.is_match(&pubkey_string);
      count+=1;
  
      print!("\r");
      print!("generated:{:?} ⚡️{} acc/s", count, count/(1+now.elapsed().as_secs()));
    }
  
    println!("\nnew account address = {}", pubkey_string);
}

fn main() {
  gen_pubkey(r"^avar.\w+")
}
