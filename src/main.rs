extern crate pinknoise;
use rand::{
    thread_rng
};

use pinknoise::{
    VmPinkRGN
};

fn main() {
    let mut rng=thread_rng();
    let mut vmpn=VmPinkRGN::<i16, 48>::new(16, &mut rng);
    for i in 0..(1_u64<<32){
        if i%(1000000)==0{
            println!("{}", i as f64/(1_u64<<32) as f64);
        }
        let x=vmpn.get(&mut rng);
        //println!("{}", x);
    }
}
