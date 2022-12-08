use sha2::{Digest, Sha256};
use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

use std::io::Write;
struct Block {
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
    next: Option<Box<Block>>,
}

impl Block {
    // private mehtod to get the current timestamp
    #[allow(dead_code)]
    fn get_timestamp() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();
        timestamp.to_string()
    }
    // generating hash
    fn generate_hash(previous_hash: String, data: String) -> String {
        let all = previous_hash + &data;
        let mut hasher = Sha256::new();
        hasher.update(all);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // constructor
    fn new(data: String, previous_hash: String) -> Block {
        Block {
            timestamp: Block::get_timestamp(),
            data: data.clone(),
            previous_hash: previous_hash.clone(),
            hash: Block::generate_hash(previous_hash, data),
            next: None,
        }
    }
}

fn search(hash: String, block: &mut Block) -> Option<&Block> {
    let mut current_block = block;
    loop {
        if current_block.hash == hash {
            return Some(current_block);
        } else if current_block.next.is_some() {
            current_block = current_block.next.as_mut().unwrap();
        } else {
            return None;
        }
    }
}

fn add(data: String, prev_hash: String) -> Block {
    let data = data.trim().to_string();
    let block = Block::new(data, prev_hash);
    println!(
        "Block created with data `{}` and hash `{}` at timestamp `{}`",
        block.data, block.hash, block.timestamp
    );
    block
}

fn main() {
    let mut genesis = Block::new("Genesis Block".to_string(), "0".to_string());
    // take input from user
    let mut data = String::new();

    let mut prev_hash = genesis.hash.clone();
    loop {
        data.clear();
        println!("================== Enter command ==================");
        // print => without the new line
        print!("=> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read line");
        if data.contains("exit") {
            break;
        } else if data.contains("add") {
            let mut read = String::new();
            let mut prev_block = &mut genesis;
            println!(" ========= Enter data to add =========");
            print!("=> ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut read)
                .expect("Failed to read line");

            let block = add(read, prev_hash);
            prev_hash = block.hash.clone();
            prev_block.next = Some(Box::new(block))
        } else if data.contains("search") {
            let mut read = String::new();
            println!(" ========= Enter hash to search =========");
            print!("=> ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut read)
                .expect("Failed to read line");
            let read = read.trim().to_string();
            let block = search(read, &mut genesis);
            match block {
                Some(block) => {
                    println!(
                        "Block found with data {} and hash {} at timestamp {}",
                        block.data, block.hash, block.timestamp
                    );
                }
                None => {
                    println!("Block not found");
                }
            }
        } else {
            println!(" Wrong command, try again");
            continue;
        }
    }
}
