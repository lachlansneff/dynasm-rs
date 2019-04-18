#![allow(unused_imports)]

#[macro_use]
extern crate dynasm;

use dynasm::{DynasmApi, dynasm};

include!("gen/prefetchwt1.rs.gen");
