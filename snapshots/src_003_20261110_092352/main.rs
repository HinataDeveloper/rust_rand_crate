// main.rs
// Date: Tue Nov 10 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning rand crate
// Goal: ...
// Dependency: Depand on `rand` crate

// rustc 1.102.0-nightly (ac8310ba6 2026-11-09)
// binary: rustc
// commit-hash: ac8310ba643a3796cbe77d2e06944d7c8513999f
// commit-date: 2026-11-09
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
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.15-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use rand::RngExt;

fn main() {
    println!("\n");

    let mut rng = rand::rng();

    if rng.random() {
        let rnd_data_one: f64 = rng.random();
        let rnd_data_two: u64 = rng.random();
        let rnd_data_three: i64 = rng.random_range(100..9000);
        println!("Random f64 is: {rnd_data_one}");
        println!("Random u64 is: {rnd_data_two}");
        println!("Random i64 is: {rnd_data_three}");
    } else {
        println!("rng.random() generated false ...");
    }

    println!("\nThe End ...\n");
}
