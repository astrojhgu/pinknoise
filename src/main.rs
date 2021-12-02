#![allow(unused_variables)]

extern crate pinknoise;
use rand::{
    thread_rng
};

use pinknoise::{
    VmPinkRng
};

fn main() {
    let mut rng=thread_rng();
    let mut vmpn=VmPinkRng::<f64, 48>::new(&mut rng);
    for i in 0..65536{
        let x=vmpn.get(&mut rng);
        println!("{}", x);
    }
}
