#![feature(asm)]
#![allow(unused_variables)]

extern crate pinknoise;
use rand::{
    thread_rng
    , Rng
};

use pinknoise::{
    VmPinkRng
    //, RandVmPinkRng
};

fn main() {
    let a:Vec<_>=(0..16).collect();
    a.iter().rev().skip(1).for_each(|x|{println!("{}",x)});
}
