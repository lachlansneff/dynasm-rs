#![allow(unused_imports)]

#[macro_use]
extern crate dynasm;

use dynasm::{DynasmApi, dynasm};

include!("gen/bmi2.rs.gen");
