use solana_sdk::{
  signature::{Keypair, Signer},
};
use regex::Regex;
use std::time::{Instant};

pub fn gen_acc(target: &str, is_regex: bool) {
  let mut pubkey_string = String::from("");
  let mut secret = String::from("");
  let mut is_matched = false;
  let mut count = 0;
  let now = Instant::now();

  while !is_matched {
    // Create new keypair
    let account = Keypair::new();
    pubkey_string = account.pubkey().to_string();
    secret = bs58::encode(account.secret().to_bytes()).into_string();

    // Matched?
    if is_regex {
      let re = Regex::new(target).unwrap();
      is_matched = re.is_match(&pubkey_string.to_owned());
    } else {
      is_matched = pubkey_string.starts_with(target);
    }

    count+=1;

    print!("\r");
    print!("generated:{:?} ⚡️{} acc/s", count, count/(1+now.elapsed().as_secs()));
  }

  println!("\npubkey: {}", pubkey_string);
  println!("secret: {}",secret);
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
 
      count+=1;
  
      print!("\r");
      print!("generated:{:?} ⚡️{} acc/s", count, count/(1+now.elapsed().as_secs()));
    }
  
    println!("\npubkey = {}", pubkey_string);
}

fn main() {
  // gen_pubkey(r"^avar.\w+", true);
  // gen_pubkey(r"dev", false);
  gen_acc(r"dev", false);
}
