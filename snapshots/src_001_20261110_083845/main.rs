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

fn main() {
    println!("\n");

    println!("\n Generate `f64` data type.");
    for index in 1..3 {
        let value: f64 = rand::random();
        println!(" -> {index} - {value}");
    }

    println!("\n Generate `u64` data type.");
    for index in 1..3 {
        let value: u64 = rand::random();
        println!(" -> {index} - {value}");
    }

    println!("\n Generate `i64` data type.");
    for index in 1..3 {
        let value: i64 = rand::random();
        println!(" -> {index} - {value}");
    }

    println!("\n Generate `i32` data type.");
    for index in 1..3 {
        let value: i32 = rand::random();
        println!(" -> {index} - {value}");
    }

    println!("\n Generate `i32` data type.");
    for index in 1..3 {
        let value: bool = rand::random();
        println!(" -> {index} - {value}");
    }

    println!("\nThe End ...\n");
}
