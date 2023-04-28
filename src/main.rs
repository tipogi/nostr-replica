use clap::Parser;
use nostr_replica::{args::Args};
use nostr_replica::services::{ backup, account };
//extern crate env_logger;

#[tokio::main]
async fn main() {

  //env_logger::init();

  match Args::parse() {
    args if args.backup => {
      let Args { followers, following, key, .. } = args;
      backup::create_backup_file(key, followers, following).await
    }
    args if args.account => {
      let Args { key, .. } = args;
      account::create_new(key).await;
    }
    _ => println!("No Action!")
  }
}
