use rand::{Rng, thread_rng};

pub fn random_num(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();

     rng.gen_range(min..max)
}
