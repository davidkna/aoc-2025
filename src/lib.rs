#![feature(int_roundings)]

use mimalloc_rust::GlobalMiMalloc;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

pub mod day01;
pub mod day02;
pub mod day03;