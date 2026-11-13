// main.rs
// Date: Wed Nov 12 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning rand crate
// Goal: Create a 5 digit number. With no repetitive and non-zero in first index.
// Dependency: Depend on `rand` crate

// Kernel Version: 7.3.3-300.fc45.x86_64

// rustc 1.102.0-nightly (3ef0c22cd 2026-11-11)
// binary: rustc
// commit-hash: 3ef0c22cdb86ac42dc6b8f0f1ec36857daa6f2a1
// commit-date: 2026-11-11
// host: x86_64-unknown-linux-gnu
// release: 1.102.0-nightly
// LLVM version: 23.1.5

// cargo 1.102.0-nightly (c07ac2c4d 2026-11-06)
// release: 1.102.0-nightly
// commit-hash: c07ac2c4d12237effc4b94f4543769b421790045
// commit-date: 2026-11-06
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 45.0.0 [64-bit]

use rand::seq::SliceRandom;

fn main() {
    println!("\n");

    let mut rng = rand::rng();
    let mut number_list: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let secret: Vec<u32> = loop {
        let (pick, _) = number_list.partial_shuffle(&mut rng, 5);
        if  let Some(1..=9) = pick.first() {
            break pick.to_vec();
        }
    };

    println!("Value of secret vector is: ");
    println!("{secret:?}");

    println!("\nThe End ...\n");
}
