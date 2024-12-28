use std::process::exit;
use std::{env, usize};

use btclib::crypto::PublicKey;
use btclib::types::Block;
use btclib::util::Saveable;

use tokio::net::TcpStream;
fn usage() -> ! {
    eprintln!(
        "Usage: {} <address> <public_key_file>",
        env::args().next().unwrap()
    );
    exit(1);
}

#[tokio::main]
async fn main() {
    let address = match env::args().nth(1) {
        Some(address) => address,
        None => usage(),
    };
    let public_key_file = match env::args().nth(2) {
        Some(pkf) => pkf,
        None => usage(),
    };

    if let Ok(publick_key) = PublicKey::load_from_file(&public_key_file) {
        println!("Connection to {address} to mine with {publick_key:?}");
    } else {
        eprintln!("Error reading public key from file {}", public_key_file);
        exit(1);
    };
}
// async fn main() {
//     let (path, steps) = if let (Some(arg), Some(arg2)) = (env::args().nth(1), env::args().nth(2)) {
//         (arg, arg2)
//     } else {
//         eprintln!("Usage: miner <block_file> <steps>");
//         exit(1)
//     };

//     let steps: usize = if let Ok(s @ 1..=usize::MAX) = steps.parse() {
//         s
//     } else {
//         eprintln!("invalid <steps> value");
//         exit(1)
//     };

//     let original_block = Block::load_from_file(path).expect("failed load block");
//     let mut block = original_block.clone();

//     println!(
//         "original block header hash: {}",
//         original_block.header.hash()
//     );

//     while !block.header.mine(steps) {
//         println!("mining")
//     }

//     println!(
//         "mined block header hash: {}",
//         format!("{:064x}", block.header.hash()),
//     );
// }
