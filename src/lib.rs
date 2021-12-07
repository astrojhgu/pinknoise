#![feature(asm)]
#![feature(proc_macro_diagnostic)]  
extern crate proc_macro;


use serde::{Deserialize, Serialize};
use std::{iter::Sum, fmt::Debug};

use num::traits::Float;

use rand::{distributions::uniform::SampleUniform, Rng};

use rand_distr::{Distribution, StandardNormal};



#[inline]
fn which_to_roll(n: usize) -> usize {
    return n.trailing_zeros() as usize
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VmPinkRng<T>
where
    T: Copy,
{
    order: usize,
    state: Vec<T>,
    cnt: usize,
    norm: T,
}

impl<T> VmPinkRng<T>
where
    T: Copy + SampleUniform + Sum + Float + Send + Sync,
    StandardNormal: Distribution<T>,
{
    pub fn from_state(state: &[T]) -> VmPinkRng<T> {
        VmPinkRng {
            order: state.len(),
            state: Vec::from(state),
            cnt: 0,
            norm: T::one() / T::from(state.len()).unwrap().sqrt(),
        }
    }

    pub fn new<R>(order: usize, rng: &mut R) -> VmPinkRng<T>
    where
        R: Rng,
    {
        let mut state = vec![T::zero(); order];
        state
            .iter_mut()
            .for_each(|x| *x = rng.sample(StandardNormal));
        Self::from_state(&state)
    }

    pub fn from_zero(order: usize) -> VmPinkRng<T> {
        Self::from_state(&vec![T::zero(); order])
    }

    pub fn get<R>(&mut self, rng: &mut R) -> T
    where
        R: Rng,
    {
        for _i in 0..2 {
            self.cnt += 1;
            let n = which_to_roll(self.cnt);
            if n < self.order {
                self.state[n] = rng.sample(StandardNormal);
            }
        }
        self.state.iter().cloned().sum::<T>() * self.norm
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VmPinkRngI<T>
where
    T: Copy,
{
    order: usize,
    state: Vec<i32>,
    cnt: usize,
    norm: T,
    range: std::ops::Range<i32>,
}

impl<T> VmPinkRngI<T>
where
    T: Copy + SampleUniform + Sum + Float + Send + Sync,
    StandardNormal: Distribution<T>,
{
    pub fn from_state(state: &[i32], r: std::ops::Range<i32>) -> Self {
        Self {
            order: state.len(),
            state: Vec::from(state),
            cnt: 0,
            norm: T::one() / T::from(state.len()).unwrap().sqrt(),
            range: r,
        }
    }

    pub fn new<R>(order: usize, r: std::ops::Range<i32>, rng: &mut R) -> Self
    where
        R: Rng,
    {
        let mut state = vec![0; order];
        state
            .iter_mut()
            .for_each(|x| *x = rng.gen_range(r.clone()));
        Self::from_state(&state, r.clone())
    }

    pub fn from_zero(order: usize, r: std::ops::Range<i32>) -> Self {
        Self::from_state(&vec![0; order], r)
    }

    pub fn get<R>(&mut self, rng: &mut R) -> T
    where
        R: Rng,
    {
        for _i in 0..2 {
            self.cnt += 1;
            let n = which_to_roll(self.cnt);
            if n < self.order {
                self.state[n] = rng.gen_range(self.range.clone());
            }
        }
        T::from(self.state.iter().cloned().sum::<i32>()).unwrap() * self.norm
    }
}



#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RandVmPinkRng<T>
where
    T: Copy,
{
    order: usize,
    pub state: Vec<T>,
    pub partial_sum: Vec<T>,
    norm: T,
    lz_upper: u32,
}

impl<T> RandVmPinkRng<T>
where
    T: Copy + SampleUniform + Sum + Float+std::fmt::Debug,
    StandardNormal: Distribution<T>,
{
    pub fn from_state(state: &[T]) -> Self {
        let order=state.len();
        let mut partial_sum=vec![T::zero(); order];
        let mut previous=T::zero();
        for (ps, &x) in partial_sum[..order-1].iter_mut().rev().zip(state[..order].iter().rev()){
            *ps=previous+x;
            previous=*ps;
        }

        Self {
            order,
            state: Vec::from(state),
            norm: T::one() / T::from(order).unwrap().sqrt(),
            lz_upper: (1_usize<<order-1).leading_zeros(),
            partial_sum
        }
    }

    pub fn new<R>(order: usize, rng: &mut R) -> Self
    where
        R: Rng,
    {
        let mut state = vec![T::zero(); order];
        
        state
            .iter_mut()
            .for_each(|x| *x = rng.sample(StandardNormal));
        Self::from_state(&state)
    }

    pub fn from_zero(order: usize) -> Self {
        Self::from_state(&vec![T::zero(); order])
    }

    #[inline]
    pub fn update_partial_sum(&mut self, n:usize){
        let mut previous=self.partial_sum[n];
        
        for (ps, &x) in self.partial_sum[..n].iter_mut().rev().zip(self.state[..=n].iter().rev()){
            *ps=previous+x;
            previous=*ps;
        }
        /*
        let x0=self.partial_sum[n];
        self.partial_sum[..n].iter_mut().rev().zip(self.state[..=n].iter().rev().scan(x0, |a,b| {
            *a=*a + *b;
            Some(*a)
        })).for_each(|(a,b)|{*a=b});*/
    }

    pub fn get<R>(&mut self, rng: &mut R) -> T
    where
        R: Rng,
    {
        let x=rng.gen_range(1_usize..(1_usize<<self.order));
        let n=(x.leading_zeros()-self.lz_upper) as usize;
        self.state[n]=rng.sample(StandardNormal);
        self.update_partial_sum(n);
        let result=(self.partial_sum[0]+self.state[0])*self.norm;
        //println!("{:?} {:?} {:?}",n, result, r1);
        result
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RandVmPinkRngI<T>
where
T: Copy + SampleUniform + Sum + Float,
{
    order: usize,
    state: Vec<i32>,
    norm: T,
    lz_upper: u32,
    range: std::ops::Range<i32>
}


impl<T> RandVmPinkRngI<T>
where
T: Copy + SampleUniform + Sum + Float,
{
    pub fn from_state(state: &[i32], r: std::ops::Range<i32>) -> Self {
        let order=state.len();
        Self {
            order,
            state: Vec::from(state),
            norm: T::one() / T::from(order).unwrap().sqrt(),
            lz_upper: (1_usize<<order-1).leading_zeros()
            , range: r
        }
    }

    pub fn new<R>(order: usize,r: std::ops::Range<i32>, rng: &mut R) -> Self
    where
        R: Rng,
    {
        let mut state = vec![0; order];
        state
            .iter_mut()
            .for_each(|x| *x = rng.gen_range(r.clone()));
        Self::from_state(&state, r)
    }

    pub fn from_zero(order: usize, r: std::ops::Range<i32>) -> Self {
        Self::from_state(&vec![0; order], r)
    }

    pub fn get<R>(&mut self, rng: &mut R) -> T
    where
        R: Rng,
        T: Float
    {
        for _i in 0..1 {
            let x=rng.gen_range(1_usize..(1_usize<<self.order));
            let n=(x.leading_zeros()-self.lz_upper) as usize;
            self.state[n]=rng.gen_range(self.range.clone());
        }
        T::from(self.state.iter().cloned().sum::<i32>()).unwrap() * self.norm
    }
}
