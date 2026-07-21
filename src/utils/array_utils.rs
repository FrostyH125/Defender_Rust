use rand::{RngExt, rngs::ThreadRng};

/// maybe inefficient idk, but its only ever used during world gen
pub fn shuffle_vec<T>(vec: &Vec<T>, rng: &mut ThreadRng) -> Vec<T>
where
    T: Clone,
{
    let mut result = vec.clone();

    for i in (0..result.len()).rev() {
        let j = rng.random_range(0..=i);
        result.swap(i, j);
    }

    return result;
}
