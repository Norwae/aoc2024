use crate::*;

// NOTES:
// 48271 is prime
// 16807 is a power of prime (7^5)
// 2147483647 is prime
// => It's a PRNG

const MODULE: u64 = 2147483647;
struct LinearCongruencePseudoRNG<const MULTIPLIER: u64, const REQUIRED_ZERO_BITS: usize> {
    seed: u64,
}

impl <const MULTIPLIER: u64, const REQUIRED_ZERO_BITS: usize> LinearCongruencePseudoRNG<MULTIPLIER, REQUIRED_ZERO_BITS> {
    const REQUIRED_ZERO_MASKS: [u64; 8] = [
        !0x0, !0x1, !0x3, !0x7,
        !0xf, !0x1f, !0x3f, !0x7f,
    ];
    fn new(seed: u64) -> Self {
        Self { seed }
    }
    fn next(&mut self) -> u16 {
        loop {
            self.seed = (self.seed * MULTIPLIER) % MODULE;

            if self.seed & Self::REQUIRED_ZERO_MASKS[REQUIRED_ZERO_BITS] == self.seed {
                return self.seed as u16
            }
        }
    }
}

simple_day!(|input|{
    let mut starts = input.lines().map(|s|{
        let space = s.rfind(" ").unwrap();
        (&s[space + 1..]).parse::<u64>().unwrap()
    });
    let seed_a = starts.next().unwrap();
    let seed_b = starts.next().unwrap();

    let mut rng1 = LinearCongruencePseudoRNG::<16807, 0>::new(seed_a);
    let mut rng2 = LinearCongruencePseudoRNG::<48271, 0>::new(seed_b);

    let mut count1 = 0;
    for _ in 0..40_000_000 {
        if rng1.next() == rng2.next() {
            count1 += 1
        }
    }

    let mut count2 = 0;
    let mut rng1 = LinearCongruencePseudoRNG::<16807, 2>::new(seed_a);
    let mut rng2 = LinearCongruencePseudoRNG::<48271, 3>::new(seed_b);

    for _ in 0..5_000_000 {
        if rng1.next() == rng2.next() {
            count2 += 1
        }
    }
    format!("Count 1: {count1}, Count 2: {count2}")
});