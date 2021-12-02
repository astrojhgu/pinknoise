#![feature(asm)]
use serde::{Deserialize, Serialize};
use std::iter::Sum;

use num::traits::Float;

use rand::{distributions::uniform::SampleUniform, Rng};

use rand_distr::{Distribution, StandardNormal};


#[cfg(target_arch = "arm")]
fn which_to_roll(mut n: usize) -> usize {
    let mut x = 0;
    while n & 1 == 0 {
        x += 1;
        n >>= 1;
    }
    x
}

#[cfg(target_arch = "x86_64")]
fn which_to_roll(n: usize) -> usize {
    let ret: u64;
    unsafe {
        asm!(
            "tzcnt rax, rax",
            in("rax") n as u64, // syscall number
            lateout("rax") ret, // clobbered by syscalls
        );
    }
    ret as usize
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
    T: Copy + SampleUniform + Sum + Float,
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
pub struct RandVmPinkRng<T>
where
    T: Copy,
{
    order: usize,
    state: Vec<T>,
    norm: T,
}

impl<T> RandVmPinkRng<T>
where
    T: Copy + SampleUniform + Sum + Float,
    StandardNormal: Distribution<T>,
{
    pub fn from_state(state: &[T]) -> Self {
        Self {
            order: state.len(),
            state: Vec::from(state),
            norm: T::one() / T::from(state.len()).unwrap().sqrt(),
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

    pub fn get<R>(&mut self, rng: &mut R) -> T
    where
        R: Rng,
    {
        for _i in 0..2 {
            for j in 0..self.order{
                if rng.gen_range(0..(1_usize<<(j+1)))==0{
                    self.state[j]=rng.sample(StandardNormal);
                }
            }
        }
        self.state.iter().cloned().sum::<T>() * self.norm
    }
}
