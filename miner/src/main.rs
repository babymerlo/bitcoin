use std::process::exit;
use std::{env, usize};

use btclib::types::Block;
use btclib::util::Saveable;

fn main() {
    let (path, steps) = if let (Some(arg), Some(arg2)) = (env::args().nth(1), env::args().nth(2)) {
        (arg, arg2)
    } else {
        eprintln!("Usage: miner <block_file> <steps>");
        exit(1)
    };

    let steps: usize = if let Ok(s @ 1..=usize::MAX) = steps.parse() {
        s
    } else {
        eprintln!("invalid <steps> value");
        exit(1)
    };

    let original_block = Block::load_from_file(path).expect("failed load block");
    let mut block = original_block.clone();

    println!(
        "original block header hash: {}",
        original_block.header.hash()
    );

    while !block.header.mine(steps) {
        println!("mining")
    }

    println!(
        "mined block header hash: {}",
        format!("{:064x}", block.header.hash()),
    );
}
