use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

pub struct Rng {
    min: f32,
    max: f32,
    seed: u64
}

impl Rng {
    pub fn new() -> Self {
        Self {
            min: 0.,
            max: 1.,
            seed: 0
        }
    }
    pub fn set_min(&mut self, val: f32) -> &mut Self {
        self.min = val;
        self
    }
    pub fn set_max(&mut self, val: f32) -> &mut Self {
        self.max = val;
        self
    }
    // taken from https://blog.orhun.dev/zero-deps-random-in-rust/
    pub fn reseed(&mut self){
        self.seed = RandomState::new().build_hasher().finish();
    }
    // taken from the following
    // https://doi.org/10.18637/jss.v008.i14
    // https://arxiv.org/pdf/1402.6246v4
    pub fn next(&mut self) -> f32 {
        if self.seed == 0 {
            self.reseed()
        }
        
        let mut val = self.seed;
        val ^= val >> 12;
        val ^= val << 25;
        val ^= val >> 27;
        val = val.wrapping_mul(2685821657736338717);
        
        let bits = ((val >> 11) as f32) / 9007199254740992.0;
        
        // bit of magic stolen from PJS
        bits * (self.max - self.min) + self.min
    }
}