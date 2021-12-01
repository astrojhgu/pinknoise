extern crate pinknoise;
use rand::{
    thread_rng
};

use pinknoise::{
    VmPinkRGN
};

fn main() {
    let mut rng=thread_rng();
    let mut vmpn=VmPinkRGN::<i16, 8>::new(16, &mut rng);
    for _ in 0..65536{
        let x=vmpn.get(&mut rng);
        println!("{}", x);
    }
    
}
