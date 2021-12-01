use std::{
    iter::{
        Sum
    }
};

use num::{
    traits::{
        PrimInt
        , Signed
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


fn which_to_roll(mut n: usize)->usize{
    if n==0{
        0
    }else{
        let mut x=0;
        while n&1==0{
            x+=1;
            n>>=1;
        }
        x
    }
}


#[derive(Clone, Copy)]
pub struct VmPinkRGN<T,const N:usize>
where T: Copy
{
    state: [T;N],
    dice_max: T,
    pub max_output: T,
    cnt: usize,
}

impl<T, const N: usize> VmPinkRGN<T, N>
where T:PrimInt+Copy+SampleUniform+Sum+Signed
{
    pub fn new<R>(dice_max: T, rng: &mut R)->VmPinkRGN<T, N>
    where R: Rng
    {
        let mut state=[T::zero();N];
        state.iter_mut().for_each(|x| *x=rng.gen_range(T::zero()..dice_max));
        VmPinkRGN{
            state, 
            dice_max, 
            max_output: T::from(N).unwrap()*dice_max,
            cnt: 0
        }
    }

    pub fn get<R>(&mut self, rng: &mut R)->T
    where  R: Rng
    {
        for i in 0..2{
            self.cnt+=1;
            let n=which_to_roll(self.cnt);
            if n<N{
                self.state[n]=rng.gen_range(T::zero()..self.dice_max);
            }    
        }
        self.state.iter().cloned().sum::<T>()-(self.max_output>>1)
    }
}
