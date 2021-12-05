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
    let mut rng = thread_rng();
    let order=4;
    for i in 0..65536*8{
        let x=rng.gen_range(1_usize..(1_usize<<order));
        let z=(x.leading_zeros()-(1_usize<<order-1).leading_zeros());
        println!("{} {}", z,x);
    }
    
}
