#![allow(unused_imports)]

#[macro_use]
extern crate dynasm;

use dynasm::{DynasmApi, dynasm};

include!("gen/sse5.rs.gen");
