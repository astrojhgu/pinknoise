use std::{
    iter::{
        Sum
    }
};

use num::{
    traits::{
        Float
    }
};

use rand::{
    Rng, 
    distributions::{
        uniform::{
            SampleUniform
        }
    }
};


use rand_distr::{
    StandardNormal
    , Distribution
};

fn which_to_roll(mut n: usize)->usize{
    let mut x=0;
    while n&1==0{
        x+=1;
        n>>=1;
    }
    x
}


#[derive(Clone, Copy)]
pub struct VmPinkRng<T,const N:usize>
where T: Copy
{
    state: [T;N],
    cnt: usize,
    norm: T,
}

impl<T, const N: usize> VmPinkRng<T, N>
where T:Copy+SampleUniform+Sum+Float
, StandardNormal: Distribution<T>
{
    pub fn from_state(state: [T; N])->VmPinkRng<T, N>{
        VmPinkRng{
            state, 
            cnt: 0, 
            norm:T::one()/T::from(N).unwrap().sqrt()
        }
    }


    pub fn new<R>(rng: &mut R)->VmPinkRng<T, N>
    where R: Rng
    {
        let mut state=[T::zero();N];
        state.iter_mut().for_each(|x| *x=rng.sample(StandardNormal));
        Self::from_state([T::zero();N])
    }

    pub fn from_zero()->VmPinkRng<T, N>{
        Self::from_state([T::zero();N])
    }

    pub fn get<R>(&mut self, rng: &mut R)->T
    where  R: Rng
    {
        for _i in 0..2{
            self.cnt+=1;
            let n=which_to_roll(self.cnt);
            if n<N{
                self.state[n]=rng.sample(StandardNormal);
            }    
        }
        self.state.iter().cloned().sum::<T>()*self.norm
    }
}
