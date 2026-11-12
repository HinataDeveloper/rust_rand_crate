// main.rs
// Date: Wed Nov 12 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning rand crate
// Goal: Using partial_shuffle and copy_from_slice methods
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

use rand::{RngExt, seq::SliceRandom};

fn main() {
    println!("\n");

    let mut rng = rand::rng();
    let first: u32 = rng.random_range(1..=9);
    let mut rest: Vec<u32> = (0..=9).filter(|n| *n != first).collect();
    let pick = rest.partial_shuffle(&mut rng, 4);
    let mut secret = [first, 0, 0, 0, 0];
    secret[1..].copy_from_slice(pick.0);
    println!("{secret:?}");

    println!("\nThe End ...\n");
}
