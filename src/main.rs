#![feature(asm)]
#![allow(unused_variables)]

extern crate pinknoise;
use rand::thread_rng;

use pinknoise::{
    VmPinkRng
    , RandVmPinkRng
};

fn main() {
    let mut rng = thread_rng();
    let mut vmpn = VmPinkRng::<f64>::new(48, &mut rng);
    for i in 0..65536 {
        let x = vmpn.get(&mut rng);
        println!("{}", x);
    }
}
