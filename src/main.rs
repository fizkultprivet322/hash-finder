use clap::{Arg, Command};
use sha2::{Sha256, Digest};
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool};
use tokio::sync::{Notify, Mutex as TokioMutex};
use std::collections::HashSet;

#[tokio::main]
async fn main() {
    let matches = Command::new("Hash Finder")
        .about("Finds integers whose SHA-256 hash ends with N zeros.")
        .arg(
            Arg::new("N")
                .short('N')
                .value_name("N")
                .help("Number of zeros the hash should end with")
                .value_parser(clap::value_parser!(usize))
                .required(true),
        )
        .arg(
            Arg::new("F")
                .short('F')
                .value_name("F")
                .help("Number of results to find")
                .value_parser(clap::value_parser!(usize))
                .required(true),
        )
        .get_matches();

    let n = matches
        .get_one::<usize>("N")
        .expect("Please provide a valid number for N")
        .clone();
    let f = matches
        .get_one::<usize>("F")
        .expect("Please provide a valid number for F")
        .clone();

    find_hashes(n, f).await;
}

async fn find_hashes(n: usize, f: usize) {
    let results = Arc::new(TokioMutex::new(HashSet::new()));
    let notify = Arc::new(Notify::new());
    let target = Arc::new(AtomicBool::new(false));

    let mut handles = vec![];

    for i in 1..=f {
        let results = Arc::clone(&results);
        let notify = Arc::clone(&notify);
        let target = Arc::clone(&target);
        let n = n.clone();
        let f = f.clone();

        let start = tokio::task::spawn(async move {
            let mut counter: u64 = i as u64;
            let mut found = 0;

            while !target.load(std::sync::atomic::Ordering::SeqCst) {
                let hash = get_sha256_hash(counter);
                if ends_with_n_zeros(&hash, n) {
                    let mut res = results.lock().await;
                    if !res.contains(&(counter, hash.clone())) {
                        println!("{}, \"{}\"", counter, hash);
                        res.insert((counter, hash));
                        found += 1;
                    }

                    if found == f {
                        target.store(true, std::sync::atomic::Ordering::SeqCst);
                        notify.notify_one();
                    }
                }
                counter += 1;
            }
        });

        handles.push(start);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

fn get_sha256_hash(num: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn ends_with_n_zeros(hash: &str, n: usize) -> bool {
    hash.chars().rev().take(n).all(|c| c == '0')
}
