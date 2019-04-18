#![allow(unused_imports)]

#[macro_use]
extern crate dynasm;

fn main() {
    dynasm!(ops
        ; .alias test, rax
    );

    println!("Please execute: cargo test --no-fail-fast")
}
