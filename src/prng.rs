#[derive(Debug)]
pub struct Prng {
    data: u16,
}

impl Prng {
    pub fn new() -> Prng {
        Prng { data: 0 }
    }

    pub fn new_from_seed(seed: u16) -> Prng {
        let mut p = Prng::new();
        p.seed(seed);
        p
    }

    pub fn new_from_time() -> Prng {
        use std::time::Instant;
        let mut seed = 0;
        while seed == 0 || seed == 0x560a || seed == 0xe550 {
            seed = (Instant::now().elapsed().subsec_nanos() % 0x10000) as u16;
        }
        Prng::new_from_seed(seed)
    }

    fn sm64(mut i: u16) -> u16 {
        if i == 0x560a {
            // 0x560a and 0xe550 are in their own cycle of 2
            // so just in case either are given as a seed
            i = 0;
        }
        let mut s0 = (i << 8) ^ i;
        i = s0 << 8 | s0 >> 8;
        s0 = i ^ ((s0 & 0xff) << 1);
        let s1 = 0xff80 ^ (s0 >> 1);
        (if s0 & 1 == 1 { 0x8180 } else { 0x1ff4 } ^ s1)
    }

    /*
    pub fn lfsr(mut lfsr: u16) -> u16 {
        let bit = ((lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5)) & 1;
        lfsr = (lfsr >> 1) | (bit << 15);
        lfsr
    }
    */

    pub fn seed(&mut self, seed: u16) {
        self.data = seed;
    }

    fn read(&self) -> u16 {
        self.data
    }

    fn next(&self) -> u16 {
        //Prng::lfsr(self.read())
        Prng::sm64(self.read())
    }

    pub fn gen_u8(&mut self) -> u8 {
        self.data = self.next();
        (self.data & 0xff) as u8
    }
}
